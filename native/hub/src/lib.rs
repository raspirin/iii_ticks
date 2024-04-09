#![deny(clippy::unwrap_used)]

use player_thread::init_player_thread;
use tokio_with_wasm::tokio::{self};

mod audio;
#[allow(clippy::unwrap_used)]
mod messages;
mod output;
mod player;
mod player_thread;

rinf::write_interface!();

async fn main() {
    tokio::spawn(start(init_player_thread()));
}

pub async fn start(sender: std::sync::mpsc::Sender<player_thread::ThreadMessage>) {
    use messages::main::*;

    let mut dart_signal_receiver = StartButtonPressed::get_dart_signal_receiver();
    while let Some(_) = dart_signal_receiver.recv().await {
        sender
            .send(player_thread::ThreadMessage::Play(
                "assets/native/test.ogg".into(),
            ))
            // TODO: fix this unwrap
            .unwrap();
    }
}
