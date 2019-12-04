use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
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
pub async fn add(
    state: web::Data<AppState>,
    web::Json(received_items): web::Json<ReceiveItemsRequest>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    info!("Received items to store in Inventory: {:?}", &received_items);

    let result = db.send(received_items).compat().await.map_err(|_| ())?;
    match result {
        Ok(_) => {
            info!("Successfully stored items in Inventory");
            let response = ReceiveItemResponse {
                status: "success".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}