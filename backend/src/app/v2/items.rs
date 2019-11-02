
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use futures::future::ok;
use crate::app::AppState;


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUnit {
    pub unit_code: String,
    pub quantity: u32
}

#[derive(Debug, Deserialize)]
pub struct Import {
    pub order_id: u32,
    pub used: bool,
    products: Vec<CreateUnit>,
    parts: Vec<CreateUnit>
}


/// Handle POST request for adding units, both parts and products, to a
/// warehouses inventory.
pub fn add(
    state: web::Data<AppState>,
    web::Json(import): web::Json<Import>,
) -> impl Future<Item=HttpResponse, Error=()> {
    println!("Adding unit to box");
    
    let db = &state.db;
    // TODO determine if you want to put the units into a box or a pallet

    // select a warehouse to add the box/pallet to
    // create a box/pallet to put all of the units into with the warehouse (or
    // pallet) as parent
    // add the box/pallet to the database

    return ok::<_, ()>(HttpResponse::Ok().json("Success"));
}