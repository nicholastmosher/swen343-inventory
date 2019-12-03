//! Implementations of CRUD endpoints for manipulating Box entities.
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
use crate::app::AppState;
use crate::models::boxes::Box;

/// The format for a response which contains a `Box` in the body.
///
/// Note that this intentionally ignores some fields that are present on the
/// Box model, such as whether the Box is deleted. Hopefully any
/// Boxes we send as responses are not deleted.
#[derive(Debug, Serialize)]
pub struct BoxResponse {
    pub id: i32,
    pub pallet_id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
}

/// Define how to convert a `Box` entity to a `BoxResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Box> for BoxResponse {
    fn from(Box { id, pallet_id, item_condition, item_quantity, .. }: Box) -> Self {
        BoxResponse { id, pallet_id, item_condition, item_quantity }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreateBox {
    pub pallet_id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
}

/// Asynchronously handles a POST request to create a Box entity.
///
/// Implemented by sending a `CreateBox` message to the `DbExecutor` actor.
pub async fn create(
    state: web::Data<AppState>,
    web::Json(create_box): web::Json<CreateBox>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(create_box).compat().await;
    match result {
        Ok(Ok(the_box)) => Ok(HttpResponse::Ok().json(the_box)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Message type for querying all Boxes from the database.
///
/// Even though we don't need any query parameters to list items from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadBoxes;

/// Asynchronously handles a GET request to list the existing Box entities.
///
/// Implemented by sending a `ReadBoxes` message to the `DbExecutor` actor.
pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let read_boxes = ReadBoxes;

    let result = db.send(read_boxes).compat().await;
    match result {
        Ok(Ok(boxes)) => Ok(HttpResponse::Ok().json(boxes)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBox {
    pub id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
}

/// Asynchronously handles a PUT request to update an existing Box entity.
///
/// Implemented by sending an `UpdateBox` message to the `DbExecutor` actor.
pub async fn update(
    state: web::Data<AppState>,
    web::Json(update_box): web::Json<UpdateBox>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(update_box).compat().await;
    match result {
        Ok(Ok(updated_box)) => Ok(HttpResponse::Ok().json(updated_box)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Deserialize the body of a Delete request to delete an existing Box.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteBox {
    pub id: i32,
}

/// Asynchronously handles a DELETE request to delete an existing Box.
///
/// Implemented by performing an update on the Box record and setting
/// the `deleted` field to true.
pub async fn delete(
    state: web::Data<AppState>,
    web::Json(delete_box): web::Json<DeleteBox>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(delete_box).compat().await;
    match result {
        Ok(Ok(deleted_box)) => Ok(HttpResponse::Ok().json(deleted_box)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
