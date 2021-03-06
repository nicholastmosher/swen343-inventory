use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use futures::{FutureExt, TryFutureExt, compat::Future01CompatExt};
use crate::app::AppState;
use crate::app::v2::stock::{ReadStock, StockInResponse};
use crate::http::v2::manufacturing::{ReturnRequest, ReturnProducts, RepairResponse};
use crate::app::v2::order;
use crate::app::v2::order::{OrderRequest, ProductInOrder};
use crate::http::v2::support::{UpdateTicket, UpdateTicketRequest, TicketStatus};

/// Send parts to manufacturing
#[derive(Debug, Serialize, Deserialize)]
pub struct PartRequest {
    pub item_code: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductRequest {
    pub item_code: String,
    pub quantity: u32,
    pub parts: Vec<PartRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSRequest {
    pub order_id: i32,
    pub product: ProductRequest,
    pub repair: bool,
}

pub async fn return_product(
    state: web::Data<AppState>,
    web::Json(returns): web::Json<CSRequest>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let http = &state.http;
    info!("Received return request: {:?}", &returns);

    // Set the Customer Support ticket status to RECEIVED
    let cs_ticket = UpdateTicket {
        order_id: returns.order_id as u32,
        request: UpdateTicketRequest {
            status: TicketStatus::RECEIVED,
        }
    };

    let result = http.send(cs_ticket).compat().await.map_err(|_| ())?;
    if let Err(e) = result {
        error!("Failed to update ticket status for return {}: {:?}", returns.order_id, e);
    }

    let result = db.send(ReadStock).compat().await;
    let stock_response = match result.map_err(|_| ())? {
        Ok(stock_response) => stock_response,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e)),
    };

    let stock: HashMap<String, StockInResponse> = stock_response.stock.into_iter()
        .map(|item| (item.product.clone(), item))
        .collect();
    debug!("Stock: {:?}", &stock);

    // kick off a repair request to Manufacturing and return a success to Sales
    actix::spawn(manufacturing_repair(state, returns, stock).boxed().compat());
    return Ok(HttpResponse::Accepted().json(RepairResponse {
        status: "return accepted; sending to manufacturing".to_string() ,
    }))
}

pub async fn manufacturing_repair(
    state: web::Data<AppState>,
    repair: CSRequest,
    stock: HashMap<String, StockInResponse>,
) -> Result<(), ()> {
    let http = &state.http;

    // MANUFACTURING FLOW //////////////////////////////////////////////////////

    // Check if we have enough raw parts to fulfill the production order

    let recipe_parts = &repair.product.parts;

    let mut need_to_order_parts = false;
    let mut parts_to_order: HashMap<String, u32> = HashMap::new();
    for part in recipe_parts {
        match stock.get(&part.item_code) {
            // If we have none of this needed part in stock, we need to order it
            None => {
                debug!("We have none of {:?}, so we need to order {} of them", &part, part.quantity);
                need_to_order_parts = true;
                parts_to_order.insert((*part.item_code).to_string(), part.quantity);
            },
            // If we have some stock but not enough, we need to order the difference
            Some(stock_quantity) if stock_quantity.quantity < part.quantity => {
                debug!("We have {} of {:?}, but we need {} more to meet the order of {}",
                    stock_quantity.quantity, &part, part.quantity - stock_quantity.quantity, part.quantity);
                need_to_order_parts = true;
                parts_to_order.insert((*part.item_code).to_string(), part.quantity - stock_quantity.quantity);
            }
            // If we have enough stock, we don't need to order any of this part
            Some(_) => {
                debug!("We have all {} of {:?} that we need!", part.quantity, &part);
                parts_to_order.insert((*part.item_code).to_string(), 0);
            },
        }
    }

    // If we don't have enough parts, make a budget request to Accounting

    // didn't make a casting between this and returnrequest due to time
    let return_order = OrderRequest {
        order_id: repair.order_id,
        products: vec!(ProductInOrder {
            product: repair.product.item_code.clone(),
            quantity: repair.product.quantity as i32,
        })
    };

    if need_to_order_parts {
        debug!("We need to order the following parts: {:?}", &parts_to_order);
        order::accounting_order(&state, &return_order, parts_to_order).await?;
    }

    // Send a "return" request to Manufacturing to repair Products with raw parts
    let parts = if repair.repair { repair.product.parts } else { vec![] };
    let return_order = ReturnRequest {
        order_id: repair.order_id,
        products: vec![ReturnProducts {
            item_code: repair.product.item_code.clone(),
            repair: repair.repair,
            parts,
        }],
    };

    match http.send(return_order).compat().await {
        // If an error occurred, don't remove parts from inventory
        Err(_) | Ok(Err(_)) => {
            error!("Repair request to Manufacturing failed!");
        },
        // If the make request succeeded, remove parts from inventory
        Ok(_) => {
            debug!("Successfully sent repair to manufacturing");
            warn!("UNIMPLEMENTED: Remove consumed parts from inventory");
        },
    }

    Ok(())
}