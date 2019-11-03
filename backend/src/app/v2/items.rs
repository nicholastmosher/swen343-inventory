use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use futures::future::ok;
use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub struct ReceiveItemsRequest {
    pub products: Option<Vec<PartInRequest>>,
    pub parts: Option<Vec<PartInRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInRequest {
    pub order_id: Option<i32>,
    pub item_code: String,
    pub quantity: u32,
    pub refurbished: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartInRequest {
    pub item_code: String,
    pub quantity: u32,
    pub refurbished: bool,
}

#[derive(Debug, Serialize)]
pub struct ReceiveItemResponse {
    pub status: String,
}

/// Handle POST request for adding units, both parts and products, to a
/// warehouses inventory.
pub fn add(
    state: web::Data<AppState>,
    web::Json(received_items): web::Json<ReceiveItemsRequest>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;
    println!("Adding unit to box");
    
    // TODO determine if you want to put the units into a box or a pallet

    // select a warehouse to add the box/pallet to
    // create a box/pallet to put all of the units into with the warehouse (or
    // pallet) as parent
    // add the box/pallet to the database

    return ok::<_, ()>(HttpResponse::Ok().json("Success"));
}