use crate::{password::verify_password, AuthService};
use anyhow::Result;
use db::{
    models::user::User,
    schema::users::dsl::{username, users},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

impl AuthService {
    pub fn login(&self, username_: &str, password_: &str) -> Result<String> {
        let user: User = users
            .filter(username.eq(username_))
            .select(User::as_select())
            .first(&mut self.get_db())?;

        verify_password(password_, &user.password)?;

        self.create_token(user.id)
    }
}
