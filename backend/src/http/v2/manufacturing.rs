//! Defines http requests that are sent to Manufacturing and their responses

use actix::{Handler, Message};
use serde::{Deserialize, Serialize};
use crate::http::HttpExecutor;

/// A request for fetching the recipes and required parts for given products.
#[derive(Debug, Serialize)]
pub struct RecipeRequest {
    pub item_code: String,
    pub quantity: u32,
}

/// A response describing the raw parts required for the requested products.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub item_code: String,
    pub quantity: u32,
    pub required_parts: Vec<PartInRecipeResponse>,
}

/// A description of the parts needed to build a specific product.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartInRecipeResponse {
    pub item_code: String,
    pub quantity: u32,
}

impl Message for RecipeRequest {
    type Result = Result<RecipeResponse, String>;
}

impl Handler<RecipeRequest> for HttpExecutor {
    type Result = <RecipeRequest as Message>::Result;

    /// Defines how to send a `RecipeRequest` to the Manufacturing silo.
    fn handle(&mut self, recipe_request: RecipeRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.manufacturing_url;

        let recipe_response = match url {
            Some(url) => {
                let url = &format!("{}/assembly/recipeInfo", &url);

                let mut response = self.client
                    .post(url)
                    .json(&recipe_request)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e));

                debug!("Received recipe response from Manufacturing: {:?}", &response);

                let recipe_response: RecipeResponse = response?.json()
                    .map_err(|_| "failed to parse response from Manufacturing")?;

                recipe_response
            },
            None => {
                RecipeResponse {
                    item_code: recipe_request.item_code,
                    quantity: recipe_request.quantity,
                    required_parts: vec![
                        PartInRecipeResponse {
                            item_code: "part:Display".to_string(),
                            quantity: recipe_request.quantity,
                        }
                    ],
                }
            }
        };

        Ok(recipe_response)
    }
}

/// Send parts to manufacturing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartRequest {
    pub item_code: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductRequest {
    pub item_code: String,
    pub quantity: u32,
    pub parts: Vec<PartRequest>,
}

#[derive(Debug, Serialize)]
pub struct SendPartsRequest {
    pub order_id: u32,
    pub warehouse_id: String,
    pub products: Vec<ProductRequest>
}

impl Message for SendPartsRequest {
    type Result = Result<(), String>;
}

impl Handler<SendPartsRequest> for HttpExecutor {
    type Result = <SendPartsRequest as Message>::Result;

    /// Defines how to send a `SendPartsRequest` to the Manufacturing silo.
    fn handle(&mut self, req: SendPartsRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.manufacturing_url;

        match url {
            Some(url) => {
                let url = &format!("{}/assembly/make", &url);

                let mut response = self.client
                    .post(url)
                    .json(&req)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e));

                debug!("Received make response from Manufacturing: {:?}", &response);

                if !response?.status().is_success() {
                    return Err("Failed to get request".to_string())
                }
            },
            None => {
                debug!("Sent STUBBED send parts request to Manufacturing");
            }
        };

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnRequest {
    pub order_id: i32,
    pub products: Vec<ReturnProducts>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnProducts {
    pub item_code: String,
    pub parts: Vec<PartRequest>,
    pub repair: bool,
}
<<<<<<< HEAD

#[derive(Debug, Serialize)]
pub struct RepairResponse {
    pub status: String,
}

/// Send repair to manufacturing
impl Message for ReturnRequest {
    type Result = Result<(), String>;
}

impl Handler<ReturnRequest> for HttpExecutor {
    type Result = <ReturnRequest as Message>::Result;

    /// Defines how to send a `SendReturnRequest` to the Manufacturing silo.
    fn handle(&mut self, req: ReturnRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.manufacturing_url;

        match url {
            Some(url) => {
                let url = &format!("{}/assembly/returns", &url);

                let mut response = self.client
                    .post(url)
                    .json(&req)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e));

=======

#[derive(Debug, Serialize)]
pub struct RepairResponse {
    pub status: String,
}

/// Send repair to manufacturing
impl Message for ReturnRequest {
    type Result = Result<(), String>;
}

impl Handler<ReturnRequest> for HttpExecutor {
    type Result = <ReturnRequest as Message>::Result;

    /// Defines how to send a `SendReturnRequest` to the Manufacturing silo.
    fn handle(&mut self, req: ReturnRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.manufacturing_url;

        match url {
            Some(url) => {
                let url = &format!("{}/assembly/returns", &url);

                let mut response = self.client
                    .post(url)
                    .json(&req)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e));

>>>>>>> 8657e10f32cadf0775ab47bbf3c2a742ec030025
                debug!("Received repair response from Manufacturing: {:?}", &response);

                if !response?.status().is_success() {
                    return Err("Failed to get request".to_string())
                }
            },
<<<<<<< HEAD
            None => ()
=======
            None => {
                debug!("Sent STUBBED return request to Manufacturing");
            }
>>>>>>> 8657e10f32cadf0775ab47bbf3c2a742ec030025
        };

        Ok(())
    }
}