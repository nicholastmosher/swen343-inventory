use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::v2::items::{ReceiveItemsRequest, ItemInRequest};
use crate::db::DbExecutor;
use crate::models::warehouses::Warehouse;
use crate::models::pallets::{NewPallet, Pallet};
use crate::models::boxes::NewBox;

impl Message for ReceiveItemsRequest {
    type Result = Result<(), String>;
}

impl Handler<ReceiveItemsRequest> for DbExecutor {
    type Result = <ReceiveItemsRequest as Message>::Result;

    fn handle(&mut self, msg: ReceiveItemsRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::warehouses;
        use crate::schema::pallets;
        use crate::schema::boxes;
        let conn = &self.0.get().expect("should get db connection");

        // Load all of the warehouses from the database.
        let warehouses: Vec<_> = warehouses::table
            .order_by(warehouses::name)
            .load::<Warehouse>(conn)
            .expect("should get warehouses");

        // If there are no warehouses, we can't receive any items.
        if warehouses.len() == 0 {
            return Err("cannot insert items: there are no warehouses to put them in!".to_string());
        }

        let default_warehouse: Warehouse = warehouses.into_iter().nth(0).unwrap();

        let ReceiveItemsRequest { products, parts } = msg;

        // Add all products to the items list
        let mut items: Vec<_> = products.unwrap_or(Vec::new())
            .into_iter().map(|product| ItemInRequest {
                item_code: product.item_code,
                ..product
            }).collect();

        // Add all parts to the items list
        items.extend(parts.unwrap_or(Vec::new())
            .into_iter().map(|part| ItemInRequest {
                item_code: part.item_code,
                ..part
            })
        );

        // If any item group does not have a specific warehouse, assign it
        // to be placed in the default warehouse
        let items = items.into_iter().map(|mut item: ItemInRequest| {
            if item.warehouse.is_none() {
                item.warehouse = Some(default_warehouse.name.clone());
            }
            item
        });

        // When we insert items, we'll just make a new box and new pallet
        // every time, unless requirements change and tell us otherwise.

        // Boxes can hold 50 units each
        let box_capacity = 50;

        // Pallets can hold 20 boxes each
        let pallet_capacity = 20;

        let counts: Vec<(ItemInRequest, u32, u32)> = items.into_iter()
            .map(|item: ItemInRequest| {
                // The number of boxes is the number of items divided by capacity
                let mut boxes = item.quantity / box_capacity;
                if item.quantity % box_capacity != 0 { boxes += 1; }

                // The number of pallets is the number of boxes divided by capacity
                let mut pallets = boxes / pallet_capacity;
                if boxes % pallet_capacity != 0 { pallets += 1; }

                (item, boxes, pallets)
            }).collect();

        for (mut item, box_count, pallet_count) in counts {

            let pallet = NewPallet {
                item_code: item.item_code,
                warehouse_name: item.warehouse.unwrap(),
            };
            let mut ps = Vec::with_capacity(pallet_count as usize);
            for _ in 0..pallet_count {
                ps.push(pallet.clone());
            }

            let inserted_pallets = diesel::insert_into(pallets::table)
                .values(&ps)
                .get_result::<Pallet>(conn);

            let mut bs = Vec::with_capacity(box_count as usize);
            for pallet in inserted_pallets {
                while item.quantity > 0 {
                    let quantity = if item.quantity > box_capacity {
                        item.quantity -= box_capacity;
                        box_capacity
                    } else {
                        let count = item.quantity;
                        item.quantity = 0;
                        count
                    };

                    let condition = if item.refurbished {
                        "refurbished"
                    } else {
                        "new"
                    };

                    bs.push(NewBox {
                        pallet_id: pallet.id,
                        item_quantity: quantity as i32,
                        item_condition: condition.to_string(),
                    });
                }
            }

            let _inserted_boxes = diesel::insert_into(boxes::table)
                .values(&bs)
                .execute(conn)
                .map_err(|e| format!("failed to insert boxes: {:?}", e))?;
        }

        Ok(())
    }
}
