use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable, Associations};
use crate::schema::pallets;
use crate::app::pallets::CreatePallet;
use crate::models::warehouses::Warehouse;

/// An in-memory representation of a Pallet entity in the database.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Warehouse)]
#[primary_key(id)]
pub struct Pallet {
    pub id: i32,
    pub item_code: String,
    pub warehouse_id: String,
    pub deleted: bool,
}

/// The minimum information needed to create a new Pallet in the database.
#[derive(Debug, Insertable)]
#[table_name = "pallets"]
pub struct NewPallet {
    pub item_code: String,
    pub warehouse_id: String,
}

/// A CreatePallet message from the webapp may be translated to a NewPallet
impl TryFrom<CreatePallet> for NewPallet {
    type Error = String;

    fn try_from(CreatePallet { item_code, warehouse_id }: CreatePallet) -> Result<Self, Self::Error> {
        Ok(NewPallet { item_code, warehouse_id })
    }
}
