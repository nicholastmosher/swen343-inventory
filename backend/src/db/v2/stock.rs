use std::collections::HashMap;
use actix::{Message, Handler};
use diesel::prelude::*;
use serde::Deserialize;
use crate::app::v2::stock::{ReadStock, StockResponse, StockInResponse};
use crate::db::DbExecutor;
use crate::models::boxes;
use crate::models::items::Item;
use crate::models::pallets::Pallet;
use crate::models::boxes::ChangedBox;

impl Message for ReadStock {
    type Result = Result<StockResponse, String>;
}

impl Handler<ReadStock> for DbExecutor {
    type Result = <ReadStock as Message>::Result;

    fn handle(&mut self, _: ReadStock, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items;
        let conn = &self.0.get().expect("should get db connection");

        let items = items::table
            .load::<Item>(conn).expect("should load items");

        let pallets: Vec<Pallet> = Pallet::belonging_to(&items)
            .load::<Pallet>(conn).expect("should load pallets");

        let boxes = boxes::Box::belonging_to(&pallets)
            .load::<boxes::Box>(conn).expect("should load boxes");

        let grouped_boxes: Vec<Vec<boxes::Box>> = boxes.grouped_by(&pallets);

        let pallets_and_boxes: Vec<Vec<(Pallet, Vec<boxes::Box>)>> = pallets.into_iter()
            .zip(grouped_boxes)
            .grouped_by(&items);

        let result: Vec<(Item, Vec<(Pallet, Vec<boxes::Box>)>)> = items.into_iter()
            .zip(pallets_and_boxes)
            .collect();

        debug!("Calculating stock using {:#?}", &result);
        let stock = result.into_iter().map(|(item, stock)| {
            let sum = stock.iter().fold(0, |acc, (_, boxes)| {
                acc + boxes.iter().fold(0, |acc, cur| acc + cur.item_quantity)
            });
            StockInResponse {
                product: item.item_code,
                category: item.item_type,
                quantity: sum as u32,
            }
        }).collect();

        Ok(StockResponse { stock })
    }
}

#[derive(Debug, Deserialize)]
pub struct RemoveStock {
    pub stock: Vec<StockToRemove>,
}

#[derive(Debug, Deserialize)]
pub struct StockToRemove {
    pub item: String,
    pub quantity: u32,
}

impl Message for RemoveStock {
    type Result = Result<(), String>;
}

impl Handler<RemoveStock> for DbExecutor {
    type Result = <RemoveStock as Message>::Result;

    fn handle(&mut self, mut msg: RemoveStock, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items;
        use crate::schema::pallets;
        use crate::schema::boxes as db_boxes;
        let conn = &self.0.get().expect("should get db connection");

        let items = items::table
            .load::<Item>(conn).expect("should load items");

        let pallets: Vec<Pallet> = Pallet::belonging_to(&items)
            .filter(pallets::deleted.eq(false))
            .load::<Pallet>(conn).expect("should load pallets");

        let boxes = boxes::Box::belonging_to(&pallets)
            .filter(db_boxes::deleted.eq(false))
            .load::<boxes::Box>(conn).expect("should load boxes");

        // Boxes are grouped by pallet and sorted by item quantity
        let grouped_boxes: Vec<Vec<boxes::Box>> = boxes.grouped_by(&pallets);

        let pallets_and_boxes: Vec<Vec<(Pallet, Vec<boxes::Box>)>> = pallets.into_iter()
            .zip(grouped_boxes)
            .grouped_by(&items);

        let mut items_and_boxes: HashMap<String, Vec<boxes::Box>> = items.into_iter()
            .map(|item| item.item_code)
            .zip(pallets_and_boxes)
            .map(|(item, pallets)| {
                (item, pallets.into_iter().flat_map(|(_, boxes)| boxes).collect())
            }).collect();

        let stock: HashMap<String, u32> = items_and_boxes.iter().map(|(item, stock)| {
            let sum = stock.iter().fold(0, |acc, the_box| acc + the_box.item_quantity);
            (item.clone(), sum as u32)
        }).collect();

        // Check whether we have enough stock to remove
        for to_remove in msg.stock.iter() {
            if let Some(in_stock) = stock.get(&to_remove.item) {
                if to_remove.quantity > *in_stock {
                    error!("Tried to remove {} {} but we only have {}!",
                        to_remove.quantity,
                        to_remove.item,
                        in_stock);
                    // Early return if we can't remove some items
                    return Err("Cannot remove more items than are in stock!".to_string());
                }
            }
        }

        // Happy path! We have enough of everything, let's remove them

        for to_remove in msg.stock.iter_mut() {
            let boxes_with_item = items_and_boxes.get_mut(&to_remove.item).unwrap();
            boxes_with_item.sort_unstable_by(|box_a, box_b| box_a.item_quantity.cmp(&box_b.item_quantity));
            debug!("Begin removing {} of {} from boxes {:#?}",
                to_remove.quantity,
                to_remove.item,
                &boxes_with_item);

            for boxx in boxes_with_item.iter_mut() {
                // If there are no more items to remove, stop searching boxes
                if to_remove.quantity == 0 { break; }

                if to_remove.quantity > boxx.item_quantity as u32 {
                    debug!("Removing all {} items from box {}", boxx.item_quantity, boxx.id);
                    // If there are more items to remove than in the box,
                    // empty the box and decrement the remove count.
                    to_remove.quantity -= boxx.item_quantity as u32;
                    boxx.item_quantity = 0;
                    boxx.deleted = true;
                } else {
                    debug!("Last removal: Removing {} items from box {}", to_remove.quantity, boxx.id);
                    // If there are fewer or equal items to remove as are
                    // left in this box, decrement items in the box and clear removal.
                    boxx.item_quantity -= to_remove.quantity as i32;
                    to_remove.quantity = 0;
                };
            }
        }

        let update_boxes: Vec<_> = items_and_boxes.values().flatten()
            .into_iter()
            .map(|boxx| ChangedBox {
                id: boxx.id,
                item_quantity: boxx.item_quantity,
                item_condition: boxx.item_condition.clone(),
            })
            .collect();

        debug!("Boxes sending for update: {:#?}", update_boxes);
        for boxx in update_boxes.iter() {
            debug!("Attempting to update box {:?}", boxx);
            diesel::update(db_boxes::table)
                .set(boxx)
                .get_result::<boxes::Box>(conn)
                .map_err(|e| format!("failed to update boxes: {:?}", e))?;
        }

        info!("\"Successfully\" removed items!");
        Ok(())
    }
}
