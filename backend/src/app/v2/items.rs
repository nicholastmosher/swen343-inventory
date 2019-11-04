use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub struct ReceiveItemsRequest {
    pub products: Option<Vec<ItemInRequest>>,
    pub parts: Option<Vec<ItemInRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct ItemInRequest {
    pub order_id: Option<i32>,
    pub item_code: String,
    pub quantity: u32,
    pub refurbished: bool,
    pub warehouse: Option<String>,
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

    db.send(received_items)
        .and_then(|res| match res {
            Ok(_) => {
                let response = ReceiveItemResponse {
                    status: "success".to_string(),
                };
                Ok(HttpResponse::Ok().json(response))
            },
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}