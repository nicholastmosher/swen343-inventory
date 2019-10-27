use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::warehouses;
use crate::app::v1::warehouses::{CreateWarehouse, UpdateWarehouse};

/// An in-memory representation of a Warehouse entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(name)]
pub struct Warehouse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// The minimum information needed to create a new Warehouse in the database.
#[derive(Debug, Insertable)]
#[table_name = "warehouses"]
pub struct NewWarehouse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// A CreateWarehouse message from the webapp may be translated to a NewWarehouse
impl TryFrom<CreateWarehouse> for NewWarehouse {
    type Error = String;

    fn try_from(CreateWarehouse { name, address, description }: CreateWarehouse) -> Result<Self, Self::Error> {
        Ok(NewWarehouse { name, address, description })
    }
}

/// A representation of changes to a Item entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "warehouses"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedWarehouse {
    pub name: String,
    pub address: String,
    pub description: Option<String>,
}

/// An UpdateWarehouse message from the webapp may be translated to a ChangedWarehouse
impl TryFrom<UpdateWarehouse> for ChangedWarehouse {
    type Error = String;

    fn try_from(UpdateWarehouse { name, address, description }: UpdateWarehouse) -> Result<Self, Self::Error> {
        Ok(ChangedWarehouse { name, address, description })
    }
}
