table! {
    collections (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        user_id -> Int4,
        public -> Bool,
    }
}

table! {
    collections_products_relation (id) {
        id -> Int4,
        collection_id -> Int4,
        product_id -> Int4,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
    }
}

table! {
    offers (id) {
        id -> Int4,
        url -> Text,
    }
}

table! {
    prices (id) {
        id -> Int4,
        offer_id -> Int4,
        value -> Nullable<Float8>,
        created_at -> Timestamp,
        availability -> crate::models::price::AvailabilityMapping,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    products_offers_relation (id) {
        id -> Int4,
        product_id -> Int4,
        offer_id -> Int4,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(collections_products_relation -> collections (collection_id));

allow_tables_to_appear_in_same_query!(
    collections,
    collections_products_relation,
    notifications,
    offers,
    prices,
    products,
    products_offers_relation,
    sessions,
    users,
);
