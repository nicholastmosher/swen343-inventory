use actix::{Message, Handler, Actor};
use diesel::prelude::*;
use crate::app::v2::stock::{ReadStock, StockResponse, StockInResponse, RemoveStock};
use crate::db::DbExecutor;
use crate::models::boxes;
use crate::models::items::Item;
use crate::models::pallets::Pallet;

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

        let stock = result.into_iter().map(|(item, stock)| {
            let sum = stock.iter().fold(0, |acc, (_, boxes)| {
                acc + boxes.iter().fold(0, |acc, cur| acc + cur.item_quantity)
            });
            StockInResponse {
                product: item.item_code,
                category: item.item_type as Option<string>,
                quantity: sum as u32,
            }
        }).collect();

        Ok(StockResponse { stock })
    }
}

impl Message for RemoveStock {
    type Result = Result<(), String>;
}

impl Handler<RemoveStock> for DbExecutor {
    type Result = <RemoveStock as Message>::Result;

    fn handle(&mut self, msg: RemoveStock, _: &mut Self::Context) -> Self::Result {
        warn!("Executing unimplemented RemoveStock request!");
        Ok(())
    }
}
