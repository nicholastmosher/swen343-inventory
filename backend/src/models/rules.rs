use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::rules;
use crate::app::v1::rules::{CreateRule, UpdateRules};
use crate::models::warehouses::Warehouse;

/// An in-memory representation of a Rule entity in the database.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Warehouse, foreign_key="warehouse_name")]
pub struct Rule {
    pub id: i32,
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>,
    pub deleted: bool,
}

/// The minimum information needed to create a new Rule in the database.
#[derive(Debug, Insertable)]
#[table_name = "rules"]
pub struct NewRule {
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>
}

/// A representation of changes to a Rule entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "rules"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedRule {
    pub id: i32,
    pub warehouse_name: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>
}

impl TryFrom<CreateRule> for NewRule {
    type Error = String;

    fn try_from(CreateRule { warehouse_name, item, minimum, quantity, description }: CreateRule) -> Result<Self, Self::Error> {
        Ok(NewRule { warehouse_name, item, minimum, quantity, description })
    }
}

impl TryFrom<UpdateRules> for ChangedRule {
    type Error = String;

    fn try_from(UpdateRules { id, warehouse_name, item, minimum, quantity, description }: UpdateRules) -> Result<Self, Self::Error> {
        if minimum < 0 || quantity < 0 { return Err("cannot set item quantity in a rule less than 0, or the minimum".to_string()); }
        Ok(ChangedRule { id, warehouse_name, item, minimum, quantity, description })
    }
}
