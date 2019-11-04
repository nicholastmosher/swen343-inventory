use serde::{Deserialize, Serialize};
use actix::{Message, Handler};
use crate::http::HttpExecutor;

#[derive(Debug)]
pub struct BudgetRequest;

#[derive(Debug, Deserialize)]
pub struct BudgetResponse {
    pub status: String,
    pub department: String,
    pub balance: f32,
}

impl Message for BudgetRequest {
    type Result = Result<BudgetResponse, String>;
}

impl Handler<BudgetRequest> for HttpExecutor {
    type Result = <BudgetRequest as Message>::Result;

    fn handle(&mut self, _: BudgetRequest, _: &mut Self::Context) -> Self::Result {
        let accounting_url = &self.config.accounting_url;

        let response = match accounting_url {
            Some(accounting_url) => {
                let mut response = self.client.get(accounting_url)
                    .query(&[("department", "inventory")])
                    .send()
                    .map_err(|e| format!("budget request failed: {:?}", e))?;

                let budget_response: BudgetResponse = response.json()
                    .map_err(|e| format!("failed to parse budget response: {:?}", e))?;
                budget_response
            },
            None => {
                BudgetResponse {
                    status: "SUCCESS".to_string(),
                    department: "INVENTORY".to_string(),
                    balance: 100_000.0,
                }
            }
        };

        Ok(response)
    }
}

#[derive(Debug, Serialize)]
pub struct ExpenseRequest {
    pub amount: f32,
    pub category: String,
    pub department: String,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseResponse {
    pub status: String,
    pub report_id: u32,
    pub department: String,
    pub balance: f32,
}

impl Message for ExpenseRequest {
    type Result = Result<ExpenseResponse, String>;
}

impl Handler<ExpenseRequest> for HttpExecutor {
    type Result = <ExpenseRequest as Message>::Result;

    fn handle(&mut self, msg: ExpenseRequest, _: &mut Self::Context) -> Self::Result {
        let accounting_url = &self.config.accounting_url;

        let response = match accounting_url {
            Some(accounting_url) => {
                let mut response = self.client.post(accounting_url)
                    .json(&msg)
                    .send()
                    .map_err(|e| format!("expense request failed: {:?}", e))?;

                let expense_response: ExpenseResponse = response.json()
                    .map_err(|e| format!("failed to parse expense response: {:?}", e))?;
                expense_response
            },
            None => {
                ExpenseResponse {
                    status: "SUCCESS".to_string(),
                    report_id: 1,
                    department: "INVENTORY".to_string(),
                    balance: 99_100.0,
                }
            }
        };

        Ok(response)
    }
}
