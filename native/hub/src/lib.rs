#![deny(clippy::unwrap_used)]

use rinf::debug_print;
use tokio_with_wasm::tokio;

mod audio;
#[allow(clippy::unwrap_used)]
mod messages;
mod output;
mod player;

rinf::write_interface!();

async fn main() {
    tokio::spawn(start());
}

pub async fn start() {
    use messages::main::*;

    let mut receiver = StartButtonPressed::get_dart_signal_receiver();
    while let Some(msg) = receiver.recv().await {
        let _start_button_pressed = msg.message;

        debug_print!("hello, world");
    }
}
