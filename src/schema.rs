// @generated automatically by Diesel CLI.

diesel::table! {
    account_login_sessions (id) {
        id -> Int4,
        account_id -> Int4,
        session_token -> Varchar,
        ip -> Varchar,
        agent -> Varchar,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    account_roles (account_id, role) {
        account_id -> Int4,
        role -> Varchar,
        identity_type -> Varchar,
        identity_value -> Varchar,
        credential_type -> Varchar,
        credential_value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (role) {
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account_login_sessions,
    account_roles,
    accounts,
    roles,
);
