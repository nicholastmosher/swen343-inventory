use serde::{Deserialize, Serialize};
use actix::{Message, Handler};
use crate::http::HttpExecutor;

/// A `BudgetRequest` message requires no parameters and causes the HttpExecutor
/// to send the request to Accounting.
#[derive(Debug)]
pub struct BudgetRequest;

/// The structure of the response received from Accounting.
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
        let url = &self.config.accounting_url;

        let response = match url {
            Some(url) => {
                let url = &format!("{}/budget", &url);

                let mut response = self.client.get(url)
                    .query(&[("department", "inventory")])
                    .send()
                    .map_err(|e| format!("budget request failed: {:?}", e))?;

                debug!("Received budget response from Accounting: {:?}", &response);

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

/// An `ExpenseRequest` asks Accounting to spend money on a particular expense.
#[derive(Debug, Serialize)]
pub struct ExpenseRequest {
    pub amount: f32,
    pub category: String,
    pub department: String,
}

/// After an Expense is made, we receive the following information from Accounting.
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
        let url = &self.config.accounting_url;

        let response = match url {
            Some(url) => {
                let url = &format!("{}/budget/expense", &url);
                let mut response = self.client.post(url)
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

/// A `BudgetIncreaseRequest` is used to ask for more budget when we're out of money.
#[derive(Debug, Serialize)]
pub struct BudgetIncreaseRequest {
    amount: f32,
    category: String,
    department: String,
    reason: String,
}

/// A `BudgetIncreaseResponse` tells us what petition our budget increase goes to.
#[derive(Debug, Deserialize)]
pub struct BudgetIncreaseResponse {
    petition_id: i32,
    status: String,
    message: String,
}

impl Message for BudgetIncreaseRequest {
    type Result = Result<BudgetIncreaseResponse, String>;
}

impl Handler<BudgetIncreaseRequest> for HttpExecutor {
    type Result = <BudgetIncreaseRequest as Message>::Result;

    fn handle(&mut self, msg: BudgetIncreaseRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.accounting_url;

        let response = match url {
            Some(url) => {
                let url = &format!("{}/budget/increase", &url);

                let mut response = self.client.post(url)
                    .json(&msg)
                    .send()
                    .map_err(|e| format!("failed to increase budget: {:?}", e))?;

                response.json::<BudgetIncreaseResponse>()
                    .map_err(|e| format!("failed to parse increase-budget response: {:?}", e))?
            },
            None => {
                BudgetIncreaseResponse {
                    petition_id: 1,
                    status: "SUCCESS".to_string(),
                    message: "Hooray!".to_string(),
                }
            }
        };

        Ok(response)
    }
}
