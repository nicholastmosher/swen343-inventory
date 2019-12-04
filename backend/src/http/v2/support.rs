use serde::Serialize;
use actix::{Message, Handler};
use crate::http::HttpExecutor;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize)]
pub enum TicketStatus {
    OPEN,
    SCHEDULED_FOR_RETURN,
    RECEIVED,
    FIXING,
    ORDER_SHIPPED_TO_CUSTOMER,
    REFUND_ISSUED,
    CLOSED,
}

#[derive(Debug, Serialize)]
pub struct UpdateTicketRequest {
    pub status: TicketStatus,
}

#[derive(Debug)]
pub struct UpdateTicket {
    pub order_id: u32,
    pub request: UpdateTicketRequest,
}

impl Message for UpdateTicket {
    type Result = Result<(), String>;
}

impl Handler<UpdateTicket> for HttpExecutor {
    type Result = <UpdateTicket as Message>::Result;

    fn handle(&mut self, msg: UpdateTicket, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.support_url;
        let UpdateTicket { order_id, request } = msg;

        match url {
            Some(url) => {
                let url = &format!("{}/api/v1/ticket/setStatus/{}", &url, order_id);

                let response = self.client
                    .post(url)
                    .json(&request)
                    .send()
                    .map_err(|e| format!("failed to send request to Customer Support: {:?}", e));

                debug!("Received response from Customer Support: {:?}", &response);

                if !response?.status().is_success() {
                    return Err("Failed to get request".to_string())
                }
            },
            None => {
                debug!("Sent STUBBED request to Customer Support");
            }
        };

        Ok(())
    }
}
