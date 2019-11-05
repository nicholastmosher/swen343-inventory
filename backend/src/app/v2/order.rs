use serde::{Deserialize, Serialize};
use futures::Future;
use actix_web::{web, HttpResponse};
use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    order_id: i32,
    products: Vec<ProductInOrder>,
}

#[derive(Debug, Deserialize)]
pub struct ProductInOrder {
    product: String,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    status: String,
}

pub fn place_order(
    state: web::Data<AppState>,
    web::Json(order): web::Json<OrderRequest>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    futures::future::ok(HttpResponse::Ok().json(OrderResponse { status: "success".to_string() }))
}
