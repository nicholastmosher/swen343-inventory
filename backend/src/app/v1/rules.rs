//! Implementations of CRUD endpoints for manipulating Rule entities.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
use crate::app::AppState;
use crate::models::rules::Rule;

/// The format for a response which contains an `Rule` in the body.
///
/// Note that this intentionally ignores some fields that are present on the
/// Rule model, such as whether the Rule is deleted. Hopefully any
/// Rules we send as responses are not deleted.
#[derive(Debug, Serialize)]
pub struct RuleResponse {
    pub id: i32,
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>,
}

/// Define how to convert a `Rule` entity to a `RuleResponse`.
///
/// This is where we strategically exclude the "deleted" field.
impl From<Rule> for RuleResponse {
    fn from(Rule { id, warehouse_name, item, minimum, quantity, description, .. }: Rule) -> Self {
        RuleResponse { id, warehouse_name, item, minimum, quantity, description }
    }
}

/// Deserialize the body of a Create request using exactly these fields.
#[derive(Debug, Deserialize)]
pub struct CreateRule {
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>,
}

/// Asynchronously handles a POST request to create a Rule entity.
///
/// Implemented by sending a `CreateRule` message to the `DbExecutor` actor.
pub async fn create(
    state: web::Data<AppState>,
    web::Json(create_item): web::Json<CreateRule>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(create_item).compat().await.map_err(|_| ())?;
    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Message type for querying all Rules from the database.
///
/// Even though we don't need any query parameters to list items from the
/// database, we still need a struct to act as a message to send to the
/// DbExecutor actor to perform the query.
#[derive(Debug)]
pub struct ReadRules;

/// Asynchronously handles a GET request to list the existing Rule entities.
///
/// Implemented by sending a `ReadRules` message to the `DbExecutor` actor.
pub async fn read(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let read_items = ReadRules;

    let result = db.send(read_items).compat().await.map_err(|_| ())?;
    match result {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Deserialize the body of an Update request using exactly these fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRules {
    pub id: i32,
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>,
}

/// Asynchronously handles a PUT request to update an existing Rule entity.
///
/// Implemented by sending an `UpdateRule` message to the `DbExecutor` actor.
pub async fn update(
    state: web::Data<AppState>,
    web::Json(update_items): web::Json<UpdateRules>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(update_items).compat().await.map_err(|_| ())?;
    match result {
        Ok(updated_item) => Ok(HttpResponse::Ok().json(updated_item)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Deserialize the body of a Delete request to delete an existing Rule.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteRule {
    pub id: i32,
}

/// Asynchronously handles a DELETE request to delete an existing Rule.
///
/// Implemented by performing an update on the Rule record and setting
/// the `deleted` field to true.
pub async fn delete(
    state: web::Data<AppState>,
    web::Json(delete_item): web::Json<DeleteRule>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    let result = db.send(delete_item).compat().await.map_err(|_| ())?;
    match result {
        Ok(deleted_item) => Ok(HttpResponse::Ok().json(deleted_item)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}
