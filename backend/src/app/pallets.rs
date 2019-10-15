//! Implementations of CRUD endpoints for manipulating Box entities.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;
use crate::models::pallets::Pallet;

/// The format for a response which contains a `Pallet` in the body.
///
/// Note that this intentionally ignores some fields that are present on the
/// Pallet model, such as whether the Pallet is deleted. Hopefully any
/// Pallets we send as responses are not deleted.
#[derive(Debug, Serialize)]
pub struct PalletResponse {
    pub id: i32,
    pub pallet_id: String,
}

/// Define how to convert a `Pallet` entity to a `PalletResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Pallet> for PalletResponse {
    fn from(Pallet { id, pallet_id, .. }: Pallet) -> Self {
        PalletResponse { id, pallet_id }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreatePallet {
    pub pallet_id: String,
}

/// Asynchronously handles a POST request to create a Pallet entity.
///
/// Implemented by sending a `CreatePallet` message to the `DbExecutor` actor.
pub fn create(
    state: web::Data<AppState>,
    web::Json(create_product): web::Json<CreatePallet>,
) -> impl Future<Pallet=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(create_product)
        .and_then(|res| match res {
            Ok(product) => Ok(HttpResponse::Ok().json(product)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

/// Message type for querying all Pallet from the database.
///
/// Even though we don't need any query parameters to list Pallets from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadPallets;

/// Asynchronously handles a GET request to list the existing Pallet entities.
///
/// Implemented by sending a `ReadPallets` message to the `DbExecutor` actor.
pub fn read(
    state: web::Data<AppState>,
) -> impl Future<Pallet=HttpResponse, Error=()> {
    let db = &state.db;
    let read_pallets = ReadPallets;

    db.send(read_pallets)
        .and_then(|res| match res {
            Ok(boxes) => Ok(HttpResponse::Ok().json(pallets)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e))
        })
        .map_err(|_| ())
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePallet {
    pub id: i32,
    pub pallet_id: String,
}

/// Asynchronously handles a PUT request to update an existing Pallet entity.
///
/// Implemented by sending an `UpdatePallet` message to the `DbExecutor` actor.
pub fn update(
    state: web::Data<AppState>,
    web::Json(update_product): web::Json<UpdatePallet>,
) -> impl Future<Pallet=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(update_pallet)
        .and_then(|res| match res {
            Ok(updated_box) => Ok(HttpResponse::Ok().json(updated_pallet)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

/// Deserialize the body of a Delete request to delete an existing Pallet.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletePallet {
    pub id: i32,
}

/// Asynchronously handles a DELETE request to delete an existing Pallet.
///
/// Implemented by performing an update on the Pallet record and setting
/// the `deleted` field to true.
pub fn delete(
    state: web::Data<AppState>,
    web::Json(delete_box): web::Json<DeletePallet>,
) -> impl Future<Pallet=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(delete_pallet)
        .and_then(|res| match res {
            Ok(deleted_box) => Ok(HttpResponse::Ok().json(deleted_pallet)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}
