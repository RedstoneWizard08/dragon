use crate::password::{hash_password, verify_password};

pub const TEST_PASSWORD: &str = "testPassword1!";

#[test]
pub fn does_hasher_work() {
    let hash = hash_password(TEST_PASSWORD);

    assert!(
        hash.is_ok(),
        "Hasher does not return an error for a valid password."
    );
}

#[test]
pub fn valid_passwords_work() {
    let hash = hash_password(TEST_PASSWORD).unwrap();
    let check = verify_password(TEST_PASSWORD, &hash);

    assert!(check.is_ok(), "Hasher can validate a password.");
}

#[test]
pub fn invalid_passwords_fail() {
    let hash = hash_password(TEST_PASSWORD).unwrap();
    let check = verify_password("Invalid Password!", &hash);

    assert!(
        check.is_err(),
        "Hasher throws an error for an invalid password."
    );
}
