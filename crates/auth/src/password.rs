use anyhow::{anyhow, Result};
use argon2::{Algorithm, Argon2, Params, Version};
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;

fn get_alg() -> Result<Argon2<'static>> {
    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            // 32mb memory
            24 * 1024,
            // 4 iterations
            2,
            // 2 degrees of parallelism
            1,
            // Default output length
            None,
        )
        .map_err(|v| anyhow!(v))?,
    ))
}

pub fn hash_password(input: &str) -> Result<String> {
    let alg = get_alg()?;
    let salt = SaltString::generate(OsRng);

    let hash = alg
        .hash_password(input.as_bytes(), &salt)
        .map_err(|v| anyhow!(v))?;

    Ok(hash.serialize().to_string())
}

pub fn verify_password(input: &str, hash: &str) -> Result<()> {
    let hash = PasswordHash::new(hash).map_err(|v| anyhow!(v))?;
    let algs: &[&dyn PasswordVerifier] = &[&get_alg()?];

    hash.verify_password(algs, input).map_err(|v| anyhow!(v))
}
