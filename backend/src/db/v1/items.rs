use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::v1::items::{CreateItem, ReadItems, UpdateItems, DeleteItem, ItemResponse};
use crate::db::DbExecutor;
use crate::models::items::{NewItem, Item, ChangedItem};

/// Allows the `CreateItem` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ItemResponse` or
/// a String describing why the database action failed.
impl Message for CreateItem {
    type Result = Result<ItemResponse, String>;
}

impl Handler<CreateItem> for DbExecutor {
    type Result = <CreateItem as Message>::Result;

    /// Defines how to handle the `CreateItem` message.
    ///
    /// Here we try to convert `CreateItem` into a `NewItem` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `ItemResponse` object.
    fn handle(&mut self, msg: CreateItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_item: NewItem = msg.try_into()?;
        diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)
            .map(ItemResponse::from)
            .map_err(|_| "should get inserted item".to_string())
    }
}

/// Allows the `ReadItems` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<ItemResponse>`
/// or a String describing why the database action failed.
impl Message for ReadItems {
    type Result = Result<Vec<ItemResponse>, String>;
}

impl Handler<ReadItems> for DbExecutor {
    type Result = <ReadItems as Message>::Result;

    /// Defines how to handle the `ReadItem` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Items` and convert them into `ItemResponse`
    /// instances to return.
    fn handle(&mut self, _: ReadItems, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        items
            .load::<Item>(conn)
            .map(|read_items| {
                read_items.into_iter()
                    .map(ItemResponse::from)
                    .collect()
            })
            .map_err(|_| "failed to get items".to_string())
    }
}

/// Allows the `UpdateItem` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ItemResponse`
/// or a String describing why the database action failed.
impl Message for UpdateItems {
    type Result = Result<ItemResponse, String>;
}

impl Handler<UpdateItems> for DbExecutor {
    type Result = <UpdateItems as Message>::Result;

    /// Defines how to handle the `UpdateItem` message.
    ///
    /// Here we try to convert `UpdateItem` into a `ChangedItem` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `ItemResponse` object.
    fn handle(&mut self, msg: UpdateItems, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let changed_item: ChangedItem = msg.try_into()?;
        diesel::update(items.filter(item_code.eq(&changed_item.item_code)))
            .set(&changed_item)
            .get_result::<Item>(conn)
            .map(ItemResponse::from)
            .map_err(|_| "failed to update item".to_string())
    }
}

/// Allows the `DeleteItem` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ItemResponse`
/// or a String describing why the database action failed.
impl Message for DeleteItem {
    type Result = Result<ItemResponse, String>;
}

impl Handler<DeleteItem> for DbExecutor {
    type Result = <DeleteItem as Message>::Result;

    /// Defines how to handle the `DeleteItem` message.
    ///
    /// Here we use the `id` from the `DeleteItem` message to find the
    /// specified `Item` record (if it exists) and update the `deleted`
    /// field to be true. Other queries are expected to ignore entities which
    /// have set the `deleted` flag.
    fn handle(&mut self, msg: DeleteItem, _: &mut Self::Context) -> Self::Result {
        use crate::schema::items::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::delete(items.filter(item_code.eq(msg.item_code)))
            .get_result::<Item>(conn)
            .map(ItemResponse::from)
            .map_err(|_| "failed to delete item".to_string())
    }
}
