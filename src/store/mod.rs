#[cfg(feature = "sqlcipher")]
pub mod sqlcipher;

use rusqlite::ffi::Error;
use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, Type};
use zeroize::Zeroize;

use thiserror::Error;

pub trait Store {}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("encryption key is invalid, unable to decrypt keyring")]
    BadKey,
}

impl From<StoreError> for rusqlite::Error {
    fn from(value: StoreError) -> rusqlite::Error {
        match value {
            StoreError::BadKey => {
                return rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error {
                        code: rusqlite::ffi::ErrorCode::PermissionDenied,
                        extended_code: -1,
                    },
                    Some(value.to_string()),
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct Secret {
    pub session: OwnedObjectPath, // o   session that defines how to read `value`
    pub parameters: Vec<u8>, // ay  algorithm params (the IV for the AES session, empty for plain)
    pub value: Vec<u8>,      // ay  the secret bytes (ciphertext or plaintext, per session)
    pub content_type: String, // s   e.g. "text/plain; charset=utf8"
}

// Don't derive Debug: it would print the secret into logs.
impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret")
            .field("session", &self.session)
            .field("value", &"<redacted>")
            .field("content_type", &self.content_type)
            .finish()
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}
