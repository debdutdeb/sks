#[cfg(feature = "sqlcipher")]
pub mod sqlcipher;

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
                        // FIXME: i don't know this, placeholder onlh
                        extended_code: -1,
                    },
                    Some(value.to_string()),
                );
            }
        }
    }
}
