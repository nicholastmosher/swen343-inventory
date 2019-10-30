//! Defines http requests that are sent to Manufacturing and their responses

use actix::{Handler, Message, Actor};
use serde::{Deserialize, Serialize};
use crate::http::HttpExecutor;

/// A request for fetching the recipes and required parts for given products.
#[derive(Debug, Serialize)]
pub struct RecipeRequest {
    pub products: Vec<ProductInRecipeRequest>,
}

/// A description of a single product lookup in a recipe request.
#[derive(Debug, Serialize)]
pub struct ProductInRecipeRequest {
    pub item_code: String,
    pub quantity: u32,
}

/// A response describing the raw parts required for the requested products.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub products: Vec<ProductInRecipeResponse>,
}

/// A description of a product that was requested and the parts needed to build it.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInRecipeResponse {
    pub item_code: String,
    pub quantity: u32,
    pub parts: Vec<PartsInRecipeResponse>,
}

/// A description of the parts needed to build a specific product.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartsInRecipeResponse {
    pub item_code: String,
    pub quantity: u32,
}

impl Message for RecipeRequest {
    type Result = Result<RecipeResponse, String>;
}

impl Handler<RecipeRequest> for HttpExecutor {
    type Result = <RecipeRequest as Message>::Result;

    /// Defines how to send a `RecipeRequest` to the Manufacturing silo.
    fn handle(&mut self, msg: RecipeRequest, _: &mut Self::Context) -> Self::Result {
        let manufacturing_url = &self.config.manufacturing_url;
        let mut response = self.client
            .post(manufacturing_url)
            .json(&msg)
            .send()
            .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e))?;

        let recipe_response: RecipeResponse = response.json()
            .map_err(|_| "failed to parse response from Manufacturing")?;

        Ok(recipe_response)
    }
}
