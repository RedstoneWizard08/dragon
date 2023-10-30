use std::sync::Arc;

use diesel::PgConnection;

pub mod login;
pub mod logout;
pub mod password;
pub mod register;
#[cfg(test)]
pub mod tests;
pub mod token;

#[derive(Clone)]
pub struct AuthService {
    db: Arc<PgConnection>,
}

impl AuthService {
    pub(crate) fn get_db(&self) -> PgConnection {
        Arc::into_inner(self.db.clone()).unwrap()
    }
}
