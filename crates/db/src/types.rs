use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Serverstatus"]
pub enum ServerStatus {
    Running,
    Starting,
    Stopped,
    Installing,
    Errored,
    Unknown,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Vartype"]
pub enum VarType {
    Text,
    Number,
}
