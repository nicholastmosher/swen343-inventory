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
        deleted -> Bool,
    }
}

joinable!(boxes -> pallets (pallet_id));
joinable!(pallets -> items (item_code));

allow_tables_to_appear_in_same_query!(
    boxes,
    items,
    pallets,
);
