table! {
    boxes (id) {
        id -> Int4,
        pallet_id -> Nullable<Int4>,
        item_code -> Nullable<Varchar>,
        quantity -> Int4,
        deleted -> Bool,
    }
}

table! {
    items (code) {
        code -> Varchar,
        cost -> Int4,
        description -> Nullable<Text>,
        deleted -> Bool,
    }
}

table! {
    pallets (id) {
        id -> Int4,
        deleted -> Bool,
    }
}

joinable!(boxes -> items (item_code));
joinable!(boxes -> pallets (pallet_id));

allow_tables_to_appear_in_same_query!(
    boxes,
    items,
    pallets,
);
