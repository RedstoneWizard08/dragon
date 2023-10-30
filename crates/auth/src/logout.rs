use crate::AuthService;
use anyhow::Result;
use db::schema::tokens::{dsl::tokens, value};
use diesel::{delete, ExpressionMethods, QueryDsl, RunQueryDsl};

impl AuthService {
    pub fn logout(&self, token: &str) -> Result<()> {
        delete(tokens.filter(value.eq(token))).execute(&mut self.get_db())?;

        Ok(())
    }
}
