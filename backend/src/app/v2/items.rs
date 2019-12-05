use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
use crate::app::AppState;
use crate::http::v2::sales::OrderStatusRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiveItemsRequest {
    pub products: Option<Vec<ItemInRequest>>,
    pub parts: Option<Vec<ItemInRequest>>,
}

#[derive(Debug, Clone, Deserialize)]
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
/// warehouses inventory. This is called by Manufacturing.
pub async fn add(
    state: web::Data<AppState>,
    web::Json(received_items): web::Json<ReceiveItemsRequest>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let http = &state.http;
    info!("Received items to store in Inventory: {:?}", &received_items);

    let items = received_items.clone();
    let result = db.send(received_items).compat().await.map_err(|_| ())?;
    let response = match result {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to receive items: {:?}", &e);
            return Ok(HttpResponse::InternalServerError().body(e));
        },
    };
    info!("Successfully stored items in Inventory");

    // If none of the received items are products with order IDs, return
    if items.products.is_none() {
        return Ok(HttpResponse::Ok().json(response));
    }

    let products = items.products.as_ref().unwrap();
    let mut responses = vec![];

    for product in products.iter() {
        if let Some(order_id) = product.order_id {
            let request = OrderStatusRequest {
                order_id: order_id as u32,
                status: "completed".to_string(),
            };
            let response = http.send(request).compat().await.map_err(|_| ())?;
            responses.push(response);
        }
    }

    if let Some(errored_response) = responses.iter().find(|response| response.is_err()) {
        error!("Failed to send order update to sales! {:?}", errored_response.as_ref().unwrap_err());
        return Ok(HttpResponse::Ok().json(ReceiveItemResponse {
            status: "failed".to_string(),
        }));
    }

    info!("Successfully notified Sales of fulfilled order");
    Ok(HttpResponse::Ok().json(ReceiveItemResponse {
        status: "success".to_string(),
    }))
}