use std::path::Path;

use super::Store;
use log::{info, warn};
use rusqlite::{Connection, Error, Params, Result};

pub struct SQLCipher<'a> {
    conn: Connection,
    table_name: &'a str,
}

use super::super::secrets::SecretString;

impl SQLCipher<'static> {
    pub fn new(path: impl AsRef<Path>, passphrase: SecretString) -> Result<Self, Error> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "key", passphrase.as_str())?;
        conn.query_row("select count(*) from sqlite_master", [], |r| {
            r.get::<_, i64>(0)
        })
        .map_err(|_| super::StoreError::BadKey)?;
        let s = Self {
            conn,
            table_name: "keyring",
        };
        info!("initializing keyring database");
        s.init()?;
        return Ok(s);
    }

    fn init(&self) -> Result<(), Error> {
        if !self.table_exists(self.table_name)? {
            let table_name = self.table_name;
            warn!("{table_name} table does not exist, creating");
            self.conn.execute_batch(&format!(
                "CREATE TABLE {} (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                self.table_name
            ))?;
        } else {
            info!("table exists")
        }
        Ok(())
    }

    fn table_exists(&self, name: &str) -> Result<bool, Error> {
        let exists = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
            [name],
            |row| row.get::<_, bool>(0),
        )?;
        Ok(exists)
    }
}
