use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::v1::rules::{CreateRule, ReadRules, UpdateRules, DeleteRule, RuleResponse};
use crate::db::DbExecutor;
use crate::models::rules::{NewRule, Rule, ChangedRule};

/// Allows the `CreateRule` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `RuleResponse` or
/// a String describing why the database action failed.
impl Message for CreateRule {
    type Result = Result<RuleResponse, String>;
}

impl Handler<CreateRule> for DbExecutor {
    type Result = <CreateRule as Message>::Result;

    /// Defines how to handle the `CreateRule` message.
    ///
    /// Here we try to convert `CreateRule` into a `NewRule` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `RuleResponse` object.
    fn handle(&mut self, msg: CreateRule, _: &mut Self::Context) -> Self::Result {
        use crate::schema::rules::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_rule: NewRule = msg.try_into()?;
        diesel::insert_into(rules)
            .values(&new_rule)
            .get_result::<Rule>(conn)
            .map(RuleResponse::from)
            .map_err(|_| "should get inserted rule".to_string())
    }
}

/// Allows the `ReadRules` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<RuleResponse>`
/// or a String describing why the database action failed.
impl Message for ReadRules {
    type Result = Result<Vec<RuleResponse>, String>;
}

impl Handler<ReadRules> for DbExecutor {
    type Result = <ReadRules as Message>::Result;

    /// Defines how to handle the `ReadRule` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Rules` and convert them into `RuleResponse`
    /// instances to return.
    fn handle(&mut self, _: ReadRules, _: &mut Self::Context) -> Self::Result {
        use crate::schema::rules::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        rules
            .filter(deleted.eq(false))
            .load::<Rule>(conn)
            .map(|read_boxes| {
                read_boxes.into_iter()
                    .map(RuleResponse::from)
                    .collect()
            })
            .map_err(|_| "failed to get rules".to_string())
    }
}

/// Allows the `UpdateRule` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `RuleResponse`
/// or a String describing why the database action failed.
impl Message for UpdateRules {
    type Result = Result<RuleResponse, String>;
}

impl Handler<UpdateRules> for DbExecutor {
    type Result = <UpdateRules as Message>::Result;

    /// Defines how to handle the `UpdateRule` message.
    ///
    /// Here we try to convert `UpdateRule` into a `ChangedRule` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `RuleResponse` object.
    fn handle(&mut self, msg: UpdateRules, _: &mut Self::Context) -> Self::Result {
        use crate::schema::rules::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let rule_id = msg.id;
        let changed_rule: ChangedRule = msg.try_into()?;
        diesel::update(rules.filter(id.eq(rule_id)))
            .set(&changed_rule)
            .get_result::<Rule>(conn)
            .map(RuleResponse::from)
            .map_err(|_| "failed to update box".to_string())
    }
}

/// Allows the `DeleteRule` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `RuleResponse`
/// or a String describing why the database action failed.
impl Message for DeleteRule {
    type Result = Result<RuleResponse, String>;
}

impl Handler<DeleteRule> for DbExecutor {
    type Result = <DeleteRule as Message>::Result;

    /// Defines how to handle the `DeleteRule` message.
    ///
    /// Here we use the `id` from the `DeleteRule` message to find the
    /// specified `Rule` record (if it exists) and update the `deleted`
    /// field to be true. Other queries are expected to ignore entities which
    /// have set the `deleted` flag.
    fn handle(&mut self, msg: DeleteRule, _: &mut Self::Context) -> Self::Result {
        use crate::schema::rules::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::update(rules.filter(id.eq(msg.id)))
            .set(deleted.eq(true))
            .get_result::<Rule>(conn)
            .map(RuleResponse::from)
            .map_err(|_| "failed to delete rule".to_string())
    }
}
