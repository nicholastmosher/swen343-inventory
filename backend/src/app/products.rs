use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;
use crate::models::products::Product;

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<Product> for ProductResponse {
    fn from(Product { id, name, description, .. }: Product) -> Self {
        ProductResponse { id, name, description }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>
}

pub fn create(
    state: web::Data<AppState>,
    web::Json(create_product): web::Json<CreateProduct>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(create_product)
        .and_then(|res| match res {
            Ok(product) => Ok(HttpResponse::Ok().json(product)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

#[derive(Debug)]
pub struct ReadProducts;

pub fn read(
    state: web::Data<AppState>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;
    let read_products = ReadProducts;

    db.send(read_products)
        .and_then(|res| match res {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e))
        })
        .map_err(|_| ())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub fn update(
    state: web::Data<AppState>,
    web::Json(update_product): web::Json<UpdateProduct>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(update_product)
        .and_then(|res| match res {
            Ok(updated_product) => Ok(HttpResponse::Ok().json(updated_product)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProduct {
    pub id: i32,
}

pub fn delete(
    state: web::Data<AppState>,
    web::Json(delete_product): web::Json<DeleteProduct>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(delete_product)
        .and_then(|res| match res {
            Ok(deleted_product) => Ok(HttpResponse::Ok().json(deleted_product)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}
