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

    let mut dart_signal_receiver = PlayerThreadMessage::get_dart_signal_receiver();
    while let Some(msg) = dart_signal_receiver.recv().await {
        let ty = msg.message.ty();
        match ty {
            player_thread_message::MessageType::Play => {
                let source_name = msg.message.source;
                sender
                    .send(player_thread::ThreadMessage::Play(source_name))
                    // TODO: fix this unwrap
                    .unwrap();
            }
            player_thread_message::MessageType::Pause => todo!(),
        }
    }
}
