use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
use crate::app::AppState;

#[derive(Debug)]
pub struct ReadStock;

#[derive(Debug, Serialize)]
pub struct StockResponse {
    pub stock: Vec<StockInResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StockInResponse {
    pub product: String,
    pub category: Option<String>,
    pub quantity: u32,
}

pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(ReadStock).compat().await.map_err(|_| ())?;
    match result {
        Ok(stock) => Ok(HttpResponse::Ok().json(stock)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

#[derive(Debug, Deserialize)]
pub struct RemoveStock {
    pub stock: Vec<StockToRemove>,
}

#[derive(Debug, Deserialize)]
pub struct StockToRemove {
    pub item: String,
    pub quantity: u32,
}

pub async fn remove(
    state: web::Data<AppState>,
    web::Json(stock): web::Json<RemoveStock>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    Ok(HttpResponse::Ok().finish())
}
