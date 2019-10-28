table! {
    boxes (id) {
        id -> Int4,
        pallet_id -> Int4,
        item_quantity -> Int4,
        deleted -> Bool,
    }
}

table! {
    items (code) {
        code -> Varchar,
        cost -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    pallets (id) {
        id -> Int4,
        item_code -> Varchar,
        warehouse_name -> Varchar,
        deleted -> Bool,
    }
}

table! {
    rules (id) {
        id -> Int4,
        warehouse_name -> Varchar,
        item -> Varchar,
        minimum -> Int4,
        quantity -> Int4,
        description -> Nullable<Text>,
        deleted -> Bool,
    }
}

table! {
    warehouses (name) {
        name -> Varchar,
        address -> Text,
        description -> Nullable<Text>,
    }
}

joinable!(boxes -> pallets (pallet_id));
joinable!(pallets -> items (item_code));
joinable!(pallets -> warehouses (warehouse_name));
joinable!(rules -> items (item));
joinable!(rules -> warehouses (warehouse_name));

allow_tables_to_appear_in_same_query!(
    boxes,
    items,
    pallets,
    rules,
    warehouses,
);
