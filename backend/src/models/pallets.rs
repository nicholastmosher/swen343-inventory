use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::pallets;
use crate::app::pallets::CreatePallet;

/// An in-memory representation of a Pallet entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(id)]
pub struct Pallet {
    pub id: i32,
    pub item_code: String,
    pub deleted: bool,
}

/// The minimum information needed to create a new Pallet in the database.
#[derive(Debug, Insertable)]
#[table_name = "pallets"]
pub struct NewPallet {
    pub item_code: String,
}

/// A CreatePallet message from the webapp may be translated to a NewPallet
impl TryFrom<CreatePallet> for NewPallet {
    type Error = String;

    fn try_from(CreatePallet { item_code }: CreatePallet) -> Result<Self, Self::Error> {
        Ok(NewPallet { item_code })
    }
}
