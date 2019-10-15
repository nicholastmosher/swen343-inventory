//! Implementations of CRUD endpoints for manipulating Item entities.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;
use crate::models::items::Item;

/// The format for a response which contains an `Item` in the body.
///
/// Note that this intentionally ignores some fields that are present on the
/// Item model, such as whether the Item is deleted. Hopefully any
/// Items we send as responses are not deleted.
#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub code: String,
    pub cost: u32,
    pub description: Option<String>,
}

/// Define how to convert a `Item` entity to a `ItemResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Item> for ItemResponse {
    fn from(Item { code, cost, description, .. }: Item) -> Self {
        ItemResponse { code, cost: cost as u32, description }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub code: String,
    pub cost: u32,
    pub description: Option<String>
}

/// Asynchronously handles a POST request to create a Item entity.
///
/// Implemented by sending a `CreateItem` message to the `DbExecutor` actor.
pub fn create(
    state: web::Data<AppState>,
    web::Json(create_item): web::Json<CreateItem>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(create_item)
        .and_then(|res| match res {
            Ok(item) => Ok(HttpResponse::Ok().json(item)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

/// Message type for querying all Items from the database.
///
/// Even though we don't need any query parameters to list items from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadItems;

/// Asynchronously handles a GET request to list the existing Item entities.
///
/// Implemented by sending a `ReadItems` message to the `DbExecutor` actor.
pub fn read(
    state: web::Data<AppState>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;
    let read_items = ReadItems;

    db.send(read_items)
        .and_then(|res| match res {
            Ok(items) => Ok(HttpResponse::Ok().json(items)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e))
        })
        .map_err(|_| ())
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateItems {
    pub code: String,
    pub cost: u32,
    pub description: Option<String>,
}

/// Asynchronously handles a PUT request to update an existing Item entity.
///
/// Implemented by sending an `UpdateItem` message to the `DbExecutor` actor.
pub fn update(
    state: web::Data<AppState>,
    web::Json(update_items): web::Json<UpdateItems>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(update_items)
        .and_then(|res| match res {
            Ok(updated_item) => Ok(HttpResponse::Ok().json(updated_item)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

/// Deserialize the body of a Delete request to delete an existing Item.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteItem {
    pub code: String,
}

/// Asynchronously handles a DELETE request to delete an existing Item.
///
/// Implemented by performing an update on the Item record and setting
/// the `deleted` field to true.
pub fn delete(
    state: web::Data<AppState>,
    web::Json(delete_item): web::Json<DeleteItem>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(delete_item)
        .and_then(|res| match res {
            Ok(deleted_item) => Ok(HttpResponse::Ok().json(deleted_item)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}