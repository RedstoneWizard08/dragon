use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::template_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TemplateGroup {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub description: String,
}
