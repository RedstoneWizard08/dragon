use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub id: i32,
    pub uuid: String,
    pub public: bool,
    pub name: String,
    pub ip: String,
    pub scheme: String,
    pub memory: i32,
    pub memory_overallocate: bool,
    pub storage: i32,
    pub storage_overallocate: bool,
    pub daemon_port: i32,
    pub sftp_port: i32,
}
