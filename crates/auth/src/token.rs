use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use db::{
    models::token::{NewToken, Token},
    schema::tokens,
};
use diesel::{insert_into, RunQueryDsl, SelectableHelper};

use crate::AuthService;

impl AuthService {
    /// Token version 1.0.0 as of 10/26/2023
    /// Token standard: "dat$$[version]$$[token]"
    pub const TOKEN_VERSION: &'static str = "v1.0.0";

    /// Token length is 64 characters + the prefix.
    pub const TOKEN_LENGTH: usize = 64;

    /// The available characters are the upper and
    /// lowercase alphabets, and number 0 through 9.
    pub const TOKEN_CHARSET: &'static str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    fn generate_raw_token() -> String {
        format!(
            "dat$${}$${}",
            Self::TOKEN_VERSION,
            random_string::generate(Self::TOKEN_LENGTH, Self::TOKEN_CHARSET)
        )
    }

    pub fn generate_encoded_token() -> String {
        let token = Self::generate_raw_token();

        general_purpose::STANDARD_NO_PAD.encode(token.as_bytes())
    }

    pub fn decode_token(token: String) -> Result<String> {
        let decoded = general_purpose::STANDARD_NO_PAD.decode(token)?;

        Ok(String::from_utf8(decoded)?)
    }

    pub fn create_token(&self, user_id: i32) -> Result<String> {
        let new_token = NewToken {
            user_id,
            value: &Self::generate_encoded_token(),
        };

        let token: Token = insert_into(tokens::table)
            .values(&new_token)
            .returning(Token::as_returning())
            .get_result(&mut self.get_db())?;

        Ok(token.value)
    }
}
