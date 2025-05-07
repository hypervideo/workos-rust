use serde::Serialize;

/// The algorithm used to hash a password.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PasswordHashType {
    /// Bcrypt hash.
    Bcrypt,

    /// Scrypt hash.
    Scrypt,

    /// Firebase Scrypt hash.
    FirebaseScrypt,

    /// SSHA hash.
    Ssha,

    /// PBKDF2 hash.
    Pbkdf2,
}

/// Password to set for the user.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PasswordParams<'a> {
    /// Plain text password.
    Password {
        /// The password to set for the user.
        password: &'a str,
    },
    /// Password hash.
    PasswordHash {
        /// The hashed password to set for the user.
        password_hash: &'a str,

        /// The algorithm originally used to hash the password.
        password_hash_type: PasswordHashType,
    },
}
