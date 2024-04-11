use lazy_static::lazy_static;
use rinf::debug_print;
use std::path::PathBuf;
use tokio_with_wasm::tokio::{fs, sync::Mutex};

use crate::messages::{self, main::PlatformPathMessage};

type P = Mutex<Option<PathBuf>>;

lazy_static! {
    static ref CONFIG_PATH: P = Mutex::new(None);
}

pub async fn init_persist_storage() {
    let mut rx = PlatformPathMessage::get_dart_signal_receiver();
    let mut config_path = CONFIG_PATH.lock().await;
    debug_print!("lock");
    if let Some(msg) = rx.recv().await {
        debug_print!("into if");
        let path = msg.message.config_path;
        debug_print!("path: {path}");
        if config_path.is_none() {
            config_path.replace(PathBuf::from(path));
        }
    }
    debug_print!("end of if");
    
    if let Some(x) = config_path.as_ref() {
        match fs::try_exists(x).await {
            Ok(is_exist) => {
                debug_print!("path exist? {is_exist}");
            },
            Err(e) => panic!("Fail to test if path exists: {e}"),
        }
    }
}
