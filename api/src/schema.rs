// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        api_key -> Varchar,
        created_at -> Timestamptz,
        last_used -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        is_active -> Bool,
        company_id -> Int8,
        #[max_length = 255]
        permission -> Nullable<Varchar>,
    }
}

diesel::table! {
    auth_group (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content_type_id -> Int4,
        #[max_length = 100]
        codename -> Varchar,
    }
}

diesel::table! {
    authtoken_token (key) {
        #[max_length = 40]
        key -> Varchar,
        created -> Timestamptz,
        user_id -> Int8,
    }
}

diesel::table! {
    companies (id) {
        id -> Int8,
        #[max_length = 255]
        company_name -> Varchar,
        company_address -> Nullable<Text>,
        #[max_length = 200]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        sending_domain -> Nullable<Varchar>,
        #[max_length = 255]
        default_from_name -> Nullable<Varchar>,
        #[max_length = 254]
        default_from_email -> Nullable<Varchar>,
        owner_id -> Int8,
        industry_id -> Nullable<Int8>,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        #[max_length = 200]
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int8,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        #[max_length = 100]
        app_label -> Varchar,
        #[max_length = 100]
        model -> Varchar,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        #[max_length = 40]
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

diesel::table! {
    email_log (id) {
        id -> Int8,
        #[max_length = 200]
        from_email -> Varchar,
        #[max_length = 200]
        to_email -> Varchar,
        #[max_length = 200]
        subject -> Varchar,
        body -> Text,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        created_at -> Timestamptz,
        company_id -> Int8,
    }
}

diesel::table! {
    industries (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    team_members (id) {
        id -> Int8,
        #[max_length = 255]
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        company_id -> Int8,
        user_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 128]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        firstname -> Nullable<Varchar>,
        #[max_length = 255]
        lastname -> Nullable<Varchar>,
        is_active -> Bool,
        is_staff -> Bool,
        mfa_enabled -> Bool,
        email_verifield -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::table! {
    users_groups (id) {
        id -> Int8,
        user_id -> Int8,
        group_id -> Int4,
    }
}

diesel::table! {
    users_user_permissions (id) {
        id -> Int8,
        user_id -> Int8,
        permission_id -> Int4,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 200]
        url -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
        last_delivered -> Nullable<Timestamptz>,
        success_rate -> Nullable<Float8>,
        events -> Nullable<Array<Nullable<Varchar>>>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        company_id -> Int8,
    }
}

diesel::joinable!(api_keys -> companies (company_id));
diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(authtoken_token -> users (user_id));
diesel::joinable!(companies -> industries (industry_id));
diesel::joinable!(companies -> users (owner_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
diesel::joinable!(django_admin_log -> users (user_id));
diesel::joinable!(email_log -> companies (company_id));
diesel::joinable!(team_members -> companies (company_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(users_groups -> auth_group (group_id));
diesel::joinable!(users_groups -> users (user_id));
diesel::joinable!(users_user_permissions -> auth_permission (permission_id));
diesel::joinable!(users_user_permissions -> users (user_id));
diesel::joinable!(webhooks -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    auth_group,
    auth_group_permissions,
    auth_permission,
    authtoken_token,
    companies,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
    email_log,
    industries,
    team_members,
    users,
    users_groups,
    users_user_permissions,
    webhooks,
);
