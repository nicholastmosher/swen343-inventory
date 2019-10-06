table! {
    products (id) {
        id -> Int4,
        name -> Text,
        code -> Text,
        price -> Int4,
        description -> Nullable<Text>,
        deleted -> Bool,
    }
}
