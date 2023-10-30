use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

use crate::types::ServerStatus;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: i32,
    pub uuid: String,
    pub uuid_short: String,
    pub owner_id: i32,
    pub node_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub template: i32,
    pub status: ServerStatus,
    pub memory: i32,
    pub storage: i32,
    pub cpu_cores: i32,
    pub port: i32,
    pub command: String,
    pub installed: Option<bool>,
}
