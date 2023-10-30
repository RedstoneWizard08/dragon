use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

use crate::types::VarType;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Template {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub docker_images: Vec<Option<String>>,
    pub install_script: String,
    pub install_image: String,
    pub install_entrypoint: String,
    pub startup_command: String,
    pub stop_command: String,
    pub startup_message: Option<String>,
    pub log_file: Option<String>,
    pub custom_log_file: bool,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::template_vars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TemplateVars {
    pub id: i32,
    pub template: i32,
    pub name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub default_value: Option<String>,
    pub editable: bool,
    pub rules: String,
    pub type_: VarType,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::template_file_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TemplateFileCOnfig {
    pub id: i32,
    pub template: i32,
    pub parser: String,
    pub find: String,
    pub replace: String,
}
