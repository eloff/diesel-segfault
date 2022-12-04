// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "citext"))]
    pub struct Citext;
}

diesel::table! {
    accounts (id) {
        id -> Uuid,
        subscription_type -> Int2,
        created_at -> Timestamptz,
        created_by -> Nullable<Int4>,
        updated_at -> Timestamptz,
        updated_by -> Nullable<Int4>,
        is_active -> Bool,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Int4,
        key -> Bytea,
        created_at -> Timestamptz,
        created_by -> Nullable<Int4>,
        updated_at -> Timestamptz,
        updated_by -> Nullable<Int4>,
        is_active -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Citext;

    users (id) {
        id -> Int4,
        email -> Citext,
        password -> Bytea,
        created_at -> Timestamptz,
        created_by -> Nullable<Int4>,
        updated_at -> Timestamptz,
        updated_by -> Nullable<Int4>,
        is_active -> Bool,
        account_id -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    api_keys,
    users,
);
