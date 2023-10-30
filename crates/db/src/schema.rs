// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "serverstatus"))]
    pub struct Serverstatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vartype"))]
    pub struct Vartype;
}

diesel::table! {
    use diesel::sql_types::*;

    nodes (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Bpchar,
        public -> Bool,
        name -> Text,
        ip -> Text,
        scheme -> Text,
        memory -> Int4,
        memory_overallocate -> Bool,
        storage -> Int4,
        storage_overallocate -> Bool,
        daemon_port -> Int4,
        sftp_port -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Serverstatus;

    servers (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Bpchar,
        #[max_length = 8]
        uuid_short -> Bpchar,
        owner_id -> Int4,
        node_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        template -> Int4,
        status -> Serverstatus,
        memory -> Int4,
        storage -> Int4,
        cpu_cores -> Int4,
        port -> Int4,
        command -> Text,
        installed -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    template_file_configs (id) {
        id -> Int4,
        template -> Int4,
        parser -> Text,
        find -> Text,
        replace -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    template_groups (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Bpchar,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Vartype;

    template_vars (id) {
        id -> Int4,
        template -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        env_var -> Nullable<Text>,
        default_value -> Nullable<Text>,
        editable -> Bool,
        rules -> Text,
        #[sql_name = "type"]
        type_ -> Vartype,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    templates (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Bpchar,
        name -> Text,
        author -> Text,
        description -> Text,
        docker_images -> Array<Nullable<Text>>,
        install_script -> Text,
        install_image -> Text,
        install_entrypoint -> Text,
        startup_command -> Text,
        stop_command -> Text,
        startup_message -> Nullable<Text>,
        log_file -> Nullable<Text>,
        custom_log_file -> Bool,
        group_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Bpchar,
        email -> Text,
        username -> Text,
        password -> Text,
        admin -> Bool,
    }
}

diesel::joinable!(servers -> nodes (node_id));
diesel::joinable!(servers -> templates (template));
diesel::joinable!(servers -> users (owner_id));
diesel::joinable!(template_file_configs -> templates (template));
diesel::joinable!(template_vars -> templates (template));
diesel::joinable!(templates -> template_groups (group_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    nodes,
    servers,
    template_file_configs,
    template_groups,
    template_vars,
    templates,
    tokens,
    users,
);
