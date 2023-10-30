use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tokens)]
pub struct NewToken<'a> {
    pub user_id: i32,
    pub value: &'a str,
}
