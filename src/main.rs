use std::{env::args, path::Path};

mod logging;
mod secrets;
mod store;

use secrets::Secret;

fn main() {
    let _ = logging::init(Path::new("sks.log"));
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        log::error!("passphrase to keyring must be passed.\n    sks <passphrase>");
        return;
    }
    let passphrase = Secret::new(args[1].as_str());
    log::info!("using passphrase {passphrase}");
    let cipher = store::sqlcipher::SQLCipher::new("sks.db", passphrase);
    match cipher {
        Err(e) => log::info!("failed to open sqlcipher {e}"),
        Ok(c) => {
            log::info!("successfully opened db")
        }
    }
}
