use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::models::warehouses::Warehouse;
use crate::app::AppState;

/// The format for a response which contains a `Warehouse` in the body.
#[derive(Debug, Serialize)]
pub struct WarehouseResponse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// Define how to convert a `Warehouse` entity to a `WarehouseResponse`.
impl From<Warehouse> for WarehouseResponse {
    fn from(Warehouse { name, address, description }: Warehouse) -> Self {
        WarehouseResponse { name, address, description }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreateWarehouse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// Asynchronously handles a POST request to create a Warehouse entity.
///
/// Implemented by sending a `CreateWarehouse` message to the `DbExecutor` actor.
pub fn create(
    state: web::Data<AppState>,
    web::Json(create_warehouse): web::Json<CreateWarehouse>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(create_warehouse)
        .and_then(|res| match res {
            Ok(warehouse) => Ok(HttpResponse::Ok().json(warehouse)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

/// Message type for querying all Warehouses from the database.
///
/// Even though we don't need any query parameters to list Warehouses from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadWarehouses;

/// Asynchronously handles a GET request to list the existing Pallet entities.
///
/// Implemented by sending a `ReadPallets` message to the `DbExecutor` actor.
pub fn read(
    state: web::Data<AppState>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;
    let read_warehouses = ReadWarehouses;

    db.send(read_warehouses)
        .and_then(|res| match res {
            Ok(warehouses) => Ok(HttpResponse::Ok().json(warehouses)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e))
        })
        .map_err(|_| ())
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct UpdateWarehouse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// Asynchronously handles a PUT request to update an existing Warehosue entity.
///
/// Implemented by sending an `UpdateWarehosue` message to the `DbExecutor` actor.
pub fn update(
    state: web::Data<AppState>,
    web::Json(update_warehouse): web::Json<UpdateWarehouse>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(update_warehouse)
        .and_then(|res| match res {
            Ok(warehouse) => Ok(HttpResponse::Ok().json(warehouse)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e))
        })
        .map_err(|_| ())
}

/// Deserialize the body of a Delete request to delete an existing Warehouse.
#[derive(Debug, Deserialize)]
pub struct DeleteWarehouse {
    pub name: String,
}

/// Asynchronously handles a DELETE request to delete an existing Item.
pub fn delete(
    state: web::Data<AppState>,
    web::Json(delete_warehouse): web::Json<DeleteWarehouse>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.db;

    db.send(delete_warehouse)
        .and_then(|res| match res {
            Ok(deleted_warehouse) => Ok(HttpResponse::Ok().json(deleted_warehouse)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}
