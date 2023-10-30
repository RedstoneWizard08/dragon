use anyhow::Result;
use db::{
    models::user::{NewUser, User},
    schema::users,
};
use diesel::{insert_into, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::AuthService;

impl AuthService {
    pub fn create_user(&self, email: &str, username: &str, password: &str) -> Result<String> {
        let uuid = Uuid::new_v4().to_string();

        let new_user = NewUser {
            uuid: uuid.as_str(),
            email,
            username,
            password,
            admin: false,
        };

        let res: User = insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.get_db())?;

        self.create_token(res.id)
    }

    pub fn create_admin_user(&self, email: &str, username: &str, password: &str) -> Result<String> {
        let uuid = Uuid::new_v4().to_string();

        let new_user = NewUser {
            uuid: uuid.as_str(),
            email,
            username,
            password,
            admin: true,
        };

        let res: User = insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.get_db())?;

        self.create_token(res.id)
    }
}
