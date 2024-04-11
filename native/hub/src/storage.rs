use std::path::PathBuf;

use directories::BaseDirs;
use lazy_static::lazy_static;
use rinf::debug_print;
use tokio_with_wasm::tokio::sync::Mutex;

use crate::messages::main::PlatformPathMessage;

type P = Mutex<Option<PathBuf>>;

lazy_static! {
    static ref CONFIG_PATH: P = Mutex::new(None);
}

pub async fn init_persist_storage() {
    use crate::messages::*;

    let mut rx = PlatformPathMessage::get_dart_signal_receiver();
    while let Some(msg) = rx.recv().await {
        let mut config_path = CONFIG_PATH.lock().await;
        if config_path.is_none() {
            let path = PathBuf::from(msg.message.config_path);
            config_path.replace(path);
        }
        break;
    }

    
}
