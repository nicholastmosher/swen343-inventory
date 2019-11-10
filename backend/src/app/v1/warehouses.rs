use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
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
pub async fn create(
    state: web::Data<AppState>,
    web::Json(create_warehouse): web::Json<CreateWarehouse>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let response = db.send(create_warehouse).compat().await;
    match response {
        Ok(Ok(warehouse)) => Ok(HttpResponse::Ok().json(warehouse)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
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
pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let read_warehouses = ReadWarehouses;

    let result = db.send(read_warehouses).compat().await;
    match result {
        Ok(Ok(warehouses)) => Ok(HttpResponse::Ok().json(warehouses)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
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
pub async fn update(
    state: web::Data<AppState>,
    web::Json(update_warehouse): web::Json<UpdateWarehouse>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(update_warehouse).compat().await;
    match result {
        Ok(Ok(warehouse)) => Ok(HttpResponse::Ok().json(warehouse)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Deserialize the body of a Delete request to delete an existing Warehouse.
#[derive(Debug, Deserialize)]
pub struct DeleteWarehouse {
    pub name: String,
}

/// Asynchronously handles a DELETE request to delete an existing Item.
pub async fn delete(
    state: web::Data<AppState>,
    web::Json(delete_warehouse): web::Json<DeleteWarehouse>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(delete_warehouse).compat().await;
    match result {
        Ok(Ok(deleted_warehouse)) => Ok(HttpResponse::Ok().json(deleted_warehouse)),
        Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
