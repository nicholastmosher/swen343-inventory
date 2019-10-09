//! Implementations of CRUD endpoints for manipulating Product entities.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;
use crate::models::products::Product;

/// The format for a response which contains a `Product` in the body.
///
/// Note that this intentionally ignores some fields that are present on the
/// Product model, such as whether the Product is deleted. Hopefully any
/// Products we send as responses are not deleted.
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price: u32,
    pub description: Option<String>,
}

/// Define how to convert a `Product` entity to a `ProductResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Product> for ProductResponse {
    fn from(Product { id, name, code, price, description, .. }: Product) -> Self {
        ProductResponse { id, name, code, price: price as u32, description }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub code: String,
    pub price: u32,
    pub description: Option<String>
}

/// Asynchronously handles a POST request to create a Product entity.
///
/// Implemented by sending a `CreateProduct` message to the `DbExecutor` actor.
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

/// Message type for querying all Products from the database.
///
/// Even though we don't need any query parameters to list items from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadProducts;

/// Asynchronously handles a GET request to list the existing Product entities.
///
/// Implemented by sending a `ReadProducts` message to the `DbExecutor` actor.
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

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price: u32,
    pub description: Option<String>,
}

/// Asynchronously handles a PUT request to update an existing Product entity.
///
/// Implemented by sending an `UpdateProduct` message to the `DbExecutor` actor.
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

/// Deserialize the body of a Delete request to delete an existing Product.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProduct {
    pub id: i32,
}

/// Asynchronously handles a DELETE request to delete an existing Product.
///
/// Implemented by performing an update on the Product record and setting
/// the `deleted` field to true.
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
