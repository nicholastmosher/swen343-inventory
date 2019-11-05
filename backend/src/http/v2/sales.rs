//! Defines http requests that are sent to Sales and their responses

use actix::{Handler, Message};
use serde::{Deserialize, Serialize};
use crate::http::HttpExecutor;


/// A description of on order status request to sales.
#[derive(Debug, Serialize)]
pub struct OrderStatusRequest {
    pub order_id: u32,
    pub status: String,
}

impl Message for OrderStatusRequest {
    type Result = Result<(), String>;
}

impl Handler<OrderStatusRequest> for HttpExecutor {
    type Result = <OrderStatusRequest as Message>::Result;

    /// Defines how to send a `RecipeRequest` to the Manufacturing silo.
    fn handle(&mut self, order_status_request: OrderStatusRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.sales_url;

        match url {
            Some(url) => {
                let url = &format!("{}/orders?status&order_id", &url);

                let mut response = self.client
                    .put(url)
                    .json(&order_status_request)
                    .send()
                    .map_err(|e| format!("failed to send request to Sales: {:?}", e))?;
            },
            None => ()
        };

        Ok(())
    }
}
