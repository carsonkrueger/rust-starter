//--- public ---
// @generated automatically by Diesel CLI.

pub mod public {}
//--- auth ---
// @generated automatically by Diesel CLI.

pub mod auth {
    diesel::table! {
        auth.participants (id) {
            id -> Uuid,
            ip_address -> Nullable<Inet>,
            user_id -> Nullable<Int8>,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        auth.privileges (id) {
            id -> Int8,
            #[max_length = 255]
            name -> Varchar,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        auth.roles (id) {
            id -> Int2,
            #[max_length = 255]
            name -> Varchar,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        auth.roles_privileges (role_id, privilege_id) {
            role_id -> Int2,
            privilege_id -> Int8,
            created_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        auth.sessions (user_id, token) {
            user_id -> Int8,
            #[max_length = 256]
            token -> Varchar,
            expires_at -> Nullable<Timestamp>,
            created_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        auth.users (id) {
            id -> Int8,
            #[max_length = 255]
            email -> Varchar,
            #[max_length = 255]
            password -> Varchar,
            #[max_length = 64]
            phone -> Nullable<Varchar>,
            #[max_length = 64]
            first_name -> Varchar,
            #[max_length = 64]
            last_name -> Varchar,
            role_id -> Int2,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
        }
    }

    diesel::joinable!(roles_privileges -> privileges (privilege_id));
    diesel::joinable!(roles_privileges -> roles (role_id));
    diesel::joinable!(sessions -> users (user_id));
    diesel::joinable!(users -> roles (role_id));

    diesel::allow_tables_to_appear_in_same_query!(
        participants,
        privileges,
        roles,
        roles_privileges,
        sessions,
        users,
    );
}
