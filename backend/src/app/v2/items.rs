
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use futures::future::ok;
use crate::app::AppState;


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub item_code: String,
    pub quantity: u32
}

#[derive(Debug, Deserialize)]
pub struct Import {
    pub order_id: u32,
    pub used: bool,
    products: Vec<CreateItem>,
    parts: Vec<CreateItem>
}


/// Handle POST request for adding items, both parts and products, to a
/// warehouses inventory.
pub fn add(
    state: web::Data<AppState>,
    web::Json(import): web::Json<Import>,
) -> impl Future<Item=HttpResponse, Error=()> {
    println!("Adding item to box");
    
    let db = &state.db;
    // TODO determine if you want to put the items into a box or a pallet

    // select a warehouse to add the box/pallet to
    // create a box/pallet to put all of the items into with the warehouse (or
    // pallet) as parent
    // add the box/pallet to the database

    return ok::<_, ()>(HttpResponse::Ok().json("Success"));
}