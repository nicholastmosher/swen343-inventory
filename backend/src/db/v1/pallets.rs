use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::v1::pallets::{CreatePallet, ReadPallets, DeletePallet, PalletResponse, UpdatePallet};
use crate::db::DbExecutor;
use crate::models::pallets::{NewPallet, Pallet, ChangePallet};

/// Allows the `CreateItem` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `PalletResponse` or
/// a String describing why the database action failed.
impl Message for CreatePallet {
    type Result = Result<PalletResponse, String>;
}

impl Handler<CreatePallet> for DbExecutor {
    type Result = <CreatePallet as Message>::Result;

    /// Defines how to handle the `CreatePallet` message.
    ///
    /// Here we try to convert `CreatePallet` into a `NewPallet` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `PalletResponse` object.
    fn handle(&mut self, msg: CreatePallet, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pallets::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_pallet: NewPallet = msg.try_into()?;
        diesel::insert_into(pallets)
            .values(&new_pallet)
            .get_result::<Pallet>(conn)
            .map(PalletResponse::from)
            .map_err(|e| format!("failed to insert pallet: {:?}", e))
    }
}

/// Allows the `ReadPallets` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<PalletResponse>`
/// or a String describing why the database action failed.
impl Message for ReadPallets {
    type Result = Result<Vec<PalletResponse>, String>;
}

impl Handler<ReadPallets> for DbExecutor {
    type Result = <ReadPallets as Message>::Result;

    /// Defines how to handle the `ReadPallets` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Pallets` and convert them into `PalletResponse`
    /// instances to return.
    fn handle(&mut self, _: ReadPallets, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pallets::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        pallets
            .filter(deleted.eq(false))
            .load::<Pallet>(conn)
            .map(|read_pallets| {
                read_pallets.into_iter()
                    .map(PalletResponse::from)
                    .collect()
            })
            .map_err(|_| "failed to get pallets".to_string())
    }
}

/// Allows the `UpdatePallet` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `PalletResponse`
/// or a String describing why the database action failed.
impl Message for UpdatePallet {
    type Result = Result<PalletResponse, String>;
}

impl Handler<UpdatePallet> for DbExecutor {
    type Result = <UpdatePallet as Message>::Result;

    /// Defines how to handle the `UpdatePallet` message.
    ///
    /// Here we try to convert `UpdatePallet` into a `ChangedPallet` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `PalletResponse` object.
    fn handle(&mut self, msg: UpdatePallet, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pallets::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let changed_pallet: ChangePallet = msg.try_into()?;
        diesel::update(pallets.filter(id.eq(&changed_pallet.id)))
            .set(&changed_pallet)
            .get_result::<Pallet>(conn)
            .map(PalletResponse::from)
            .map_err(|_| "failed to update pallet".to_string())
    }
}

/// Allows the `DeletePallet` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `PalletResponse`
/// or a String describing why the database action failed.
impl Message for DeletePallet {
    type Result = Result<PalletResponse, String>;
}

impl Handler<DeletePallet> for DbExecutor {
    type Result = <DeletePallet as Message>::Result;

    /// Defines how to handle the `DeletePallet` message.
    ///
    /// Here we use the `id` from the `DeletePallet` message to find the
    /// specified `Pallet` record (if it exists) and update the `deleted`
    /// field to be true. Other queries are expected to ignore entities which
    /// have set the `deleted` flag.
    fn handle(&mut self, msg: DeletePallet, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pallets::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::update(pallets.filter(id.eq(msg.id)))
            .set(deleted.eq(true))
            .get_result::<Pallet>(conn)
            .map(PalletResponse::from)
            .map_err(|_| "failed to delete pallets".to_string())
    }
}
