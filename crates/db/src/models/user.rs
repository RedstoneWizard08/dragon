use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub uuid: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub admin: bool,
}
