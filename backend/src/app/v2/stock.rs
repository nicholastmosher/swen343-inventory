use futures::Future;
use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::app::AppState;

#[derive(Debug)]
pub struct ReadStock;

#[derive(Debug, Serialize)]
pub struct StockResponse {
    pub stock: Vec<StockInResponse>,
}

#[derive(Debug, Serialize)]
pub struct StockInResponse {
    pub product: String,
    pub quantity: u32,
}

pub fn read(
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    let db = &state.db;

    db.send(ReadStock)
        .and_then(|res| match res {
            Ok(stock) => Ok(HttpResponse::Ok().json(stock)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}
