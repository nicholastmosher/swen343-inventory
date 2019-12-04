//! Implementations of CRUD endpoints for manipulating Box entities.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
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
    pub item_code: String,
    pub warehouse_name: String,
}

/// Define how to convert a `Pallet` entity to a `PalletResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Pallet> for PalletResponse {
    fn from(Pallet { id, item_code, warehouse_name, .. }: Pallet) -> Self {
        PalletResponse { id, item_code, warehouse_name }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreatePallet {
    pub item_code: String,
    pub warehouse_name: String,
}

/// Asynchronously handles a POST request to create a Pallet entity.
///
/// Implemented by sending a `CreatePallet` message to the `DbExecutor` actor.
pub async fn create(
    state: web::Data<AppState>,
    web::Json(create_pallet): web::Json<CreatePallet>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(create_pallet).compat().await.map_err(|_| ())?;
    match result {
        Ok(pallet) => Ok(HttpResponse::Ok().json(pallet)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
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
pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let read_pallets = ReadPallets;

    let result = db.send(read_pallets).compat().await.map_err(|_| ())?;
    match result {
        Ok(pallets) => Ok(HttpResponse::Ok().json(pallets)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct UpdatePallet {
    pub id: i32,
    pub item_code: String,
    pub warehouse_name: String,
}

/// Asynchronously handles a PUT request to update an existing Item entity.
///
/// Implemented by sending an `UpdatePallet` message to the `DbExecutor` actor.
pub async fn update(
    state: web::Data<AppState>,
    web::Json(update_pallet): web::Json<UpdatePallet>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(update_pallet).compat().await.map_err(|_| ())?;
    match result {
        Ok(pallet) => Ok(HttpResponse::Ok().json(pallet)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Deserialize the body of a Delete request to delete an existing Pallet.
#[derive(Debug, Deserialize)]
pub struct DeletePallet {
    pub id: i32,
}

/// Asynchronously handles a DELETE request to delete an existing Pallet.
///
/// Implemented by performing an update on the Pallet record and setting
/// the `deleted` field to true.
pub async fn delete(
    state: web::Data<AppState>,
    web::Json(delete_pallet): web::Json<DeletePallet>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(delete_pallet).compat().await.map_err(|_| ())?;
    match result {
        Ok(deleted_pallet) => Ok(HttpResponse::Ok().json(deleted_pallet)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}
