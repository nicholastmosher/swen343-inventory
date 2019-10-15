use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::pallets;
use crate::app::pallets::{CreatePallet, UpdatePallets};
use std::convert::TryFrom;

/// An in-memory representation of a Pallet entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(code)]
pub struct Pallet {
    pub pallet_id: String,
    pub deleted: bool,
}

/// The minimum information needed to create a new Pallet in the database.
#[derive(Debug, Insertable)]
#[table_name = "pallets"]
pub struct NewPallet {
    pub pallet_id: String,
}

/// A representation of changes to a Pallet entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "pallets"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedPallet {
    pub pallet_id: String,
}

/// A CreatePallet message from the webapp may be translated to a NewPallet
///
/// CreatePallet is directly deserialized from the web request and contains
/// a u32 for the cost. If the value of the u32 is in the range of the i32
/// values, we can define a translation of the CreatePallet message into the
/// NewPallet type used to insert a new entity into the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
impl TryFrom<CreatePallet> for NewPallet {
    type Error = String;

    fn try_from(CreatePallet { pallet_id }: CreatePallet) -> Result<Self, Self::Error> {
        // if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(NewPallet { pallet_id })
    }
}

/// An UpdatePallet message from the webapp may be translated to a ChangedPallet
///
/// UpdatePallet is directly deserialized from the web request and contains a
/// u32 for the cost. If the value of the u32 is in the range of i32 values,
/// we can define a translation of the UpdatePallet message into the
/// ChangedPallet type used to update an entity in the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
impl TryFrom<UpdatePallets> for ChangedPallet {
    type Error = String;

    fn try_from(UpdatePallets { }: UpdatePallets) -> Result<Self, Self::Error> {
        // if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(ChangedPallet { pallet_id })
    }
}
