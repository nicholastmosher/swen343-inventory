use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::v1::boxes::{CreateBox, ReadBoxes, UpdateBox, DeleteBox, BoxResponse};
use crate::db::DbExecutor;
use crate::models::boxes::{NewBox, Box, ChangedBox};

/// Allows the `CreateBox` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `BoxResponse` or
/// a String describing why the database action failed.
impl Message for CreateBox {
    type Result = Result<BoxResponse, String>;
}

impl Handler<CreateBox> for DbExecutor {
    type Result = <CreateBox as Message>::Result;

    /// Defines how to handle the `CreateBox` message.
    ///
    /// Here we try to convert `CreateBox` into a `NewBox` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `BoxResponse` object.
    fn handle(&mut self, msg: CreateBox, _: &mut Self::Context) -> Self::Result {
        use crate::schema::boxes::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_box: NewBox = msg.try_into()?;
        diesel::insert_into(boxes)
            .values(&new_box)
            .get_result::<Box>(conn)
            .map(BoxResponse::from)
            .map_err(|_| "should get inserted box".to_string())
    }
}

/// Allows the `ReadBoxes` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<BoxResponse>`
/// or a String describing why the database action failed.
impl Message for ReadBoxes {
    type Result = Result<Vec<BoxResponse>, String>;
}

impl Handler<ReadBoxes> for DbExecutor {
    type Result = <ReadBoxes as Message>::Result;

    /// Defines how to handle the `ReadBox` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Boxs` and convert them into `BoxResponse`
    /// instances to return.
    fn handle(&mut self, _: ReadBoxes, _: &mut Self::Context) -> Self::Result {
        use crate::schema::boxes::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        boxes
            .filter(deleted.eq(false))
            .load::<Box>(conn)
            .map(|read_boxes| {
                read_boxes.into_iter()
                    .map(BoxResponse::from)
                    .collect()
            })
            .map_err(|_| "failed to get boxes".to_string())
    }
}

/// Allows the `UpdateBox` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `BoxResponse`
/// or a String describing why the database action failed.
impl Message for UpdateBox {
    type Result = Result<BoxResponse, String>;
}

impl Handler<UpdateBox> for DbExecutor {
    type Result = <UpdateBox as Message>::Result;

    /// Defines how to handle the `UpdateBox` message.
    ///
    /// Here we try to convert `UpdateBox` into a `ChangedBox` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `BoxResponse` object.
    fn handle(&mut self, msg: UpdateBox, _: &mut Self::Context) -> Self::Result {
        use crate::schema::boxes::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let box_id = msg.id;
        let changed_box: ChangedBox = msg.try_into()?;
        diesel::update(boxes.filter(id.eq(box_id)))
            .set(&changed_box)
            .get_result::<Box>(conn)
            .map(BoxResponse::from)
            .map_err(|_| "failed to update box".to_string())
    }
}

/// Allows the `DeleteBox` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `BoxResponse`
/// or a String describing why the database action failed.
impl Message for DeleteBox {
    type Result = Result<BoxResponse, String>;
}

impl Handler<DeleteBox> for DbExecutor {
    type Result = <DeleteBox as Message>::Result;

    /// Defines how to handle the `DeleteBox` message.
    ///
    /// Here we use the `id` from the `DeleteBox` message to find the
    /// specified `Box` record (if it exists) and update the `deleted`
    /// field to be true. Other queries are expected to ignore entities which
    /// have set the `deleted` flag.
    fn handle(&mut self, msg: DeleteBox, _: &mut Self::Context) -> Self::Result {
        use crate::schema::boxes::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::update(boxes.filter(id.eq(msg.id)))
            .set(deleted.eq(true))
            .get_result::<Box>(conn)
            .map(BoxResponse::from)
            .map_err(|_| "failed to delete box".to_string())
    }
}
