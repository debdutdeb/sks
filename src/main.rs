use std::{env::args, path::Path};

mod logging;
mod secrets;
mod store;

use secrets::SecretString;

fn main() {
    let _ = logging::init(Path::new("sks.log"));
    let mut args = args();
    if args.len() != 2 {
        log::error!("passphrase to keyring must be passed.\n    sks <passphrase>");
        return;
    }
    let passphrase = SecretString::new(args.nth(1).unwrap());
    log::info!("using passphrase {passphrase}");
    let cipher = store::sqlcipher::SQLCipher::new("sks.db", passphrase);
    match cipher {
        Err(e) => log::error!("failed to open sqlcipher: {e}"),
        Ok(c) => {
            let _ = c;
            log::info!("successfully opened db")
        }
    }
}
