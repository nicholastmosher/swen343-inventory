use actix_web::{web, HttpResponse};
use serde::Serialize;
use futures::compat::Future01CompatExt;
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

pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(ReadStock).compat().await;
    match result {
        Ok(Ok(stock)) => Ok(HttpResponse::Ok().json(stock)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
