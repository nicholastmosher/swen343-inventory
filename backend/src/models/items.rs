use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::items;
use crate::app::v1::items::{CreateItem, UpdateItems};
use std::convert::TryFrom;

/// An in-memory representation of a Item entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(item_code)]
pub struct Item {
    pub item_code: String,
    pub item_type: Option<String>,
    pub cost: i32,
    pub description: Option<String>,
}

/// The minimum information needed to create a new Item in the database.
#[derive(Debug, Insertable)]
#[table_name = "items"]
pub struct NewItem {
    pub item_code: String,
    pub item_type: Option<String>,
    pub cost: i32,
    pub description: Option<String>,
}

/// A representation of changes to a Item entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "items"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedItem {
    pub item_code: String,
    pub item_type: Option<String>,
    pub cost: i32,
    pub description: Option<String>,
}

/// A CreateItem message from the webapp may be translated to a NewItem
///
/// CreateItem is directly deserialized from the web request and contains
/// a u32 for the cost. If the value of the u32 is in the range of the i32
/// values, we can define a translation of the CreateItem message into the
/// NewItem type used to insert a new entity into the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
impl TryFrom<CreateItem> for NewItem {
    type Error = String;

    fn try_from(CreateItem { item_code, item_type, cost, description }: CreateItem) -> Result<Self, Self::Error> {
        if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(NewItem { item_code, item_type, cost: cost as i32, description })
    }
}

/// An UpdateItem message from the webapp may be translated to a ChangedItem
///
/// UpdateItem is directly deserialized from the web request and contains a
/// u32 for the cost. If the value of the u32 is in the range of i32 values,
/// we can define a translation of the UpdateItem message into the
/// ChangedItem type used to update an entity in the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
impl TryFrom<UpdateItems> for ChangedItem {
    type Error = String;

    fn try_from(UpdateItems { item_code, item_type, cost, description }: UpdateItems) -> Result<Self, Self::Error> {
        if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(ChangedItem { item_code, item_type, cost: cost as i32, description })
    }
}
