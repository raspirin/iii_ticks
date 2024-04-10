use std::sync::mpsc::{Receiver, TryRecvError};

use rinf::debug_print;

use crate::player::{Player, PlayerError};

pub enum ThreadMessage {
    Play(String),
    Pause,
}

/// Init a sync thread which listen to tokio thread and holds the player.
pub fn init_player_thread() -> std::sync::mpsc::Sender<ThreadMessage> {
    let (tx, rx) = std::sync::mpsc::channel::<ThreadMessage>();
    std::thread::spawn(move || {
        thread(rx, |e| {
            debug_print!("{e}");
        })
    });
    tx
}

pub fn thread(rx: Receiver<ThreadMessage>, error_fn: impl FnOnce(PlayerError) + std::marker::Copy) {
    let mut player: Option<Player> = None;
    let receiver = rx;
    loop {
        match receiver.try_recv() {
            Ok(ThreadMessage::Pause) => {
                player.as_mut().map(|x| x.pause());
            }
            Ok(ThreadMessage::Play(path)) => {
                if player.is_none() {
                    player = match Player::new(&path) {
                        Ok(p) => Some(p),
                        Err(e) => {
                            error_fn(e);
                            None
                        }
                    }
                } else {
                    player.as_mut().map(|x| {
                        if let Err(e) = x.open(&path) {
                            error_fn(e)
                        }
                    });
                }
            }
            Err(TryRecvError::Empty) => {
                player.as_mut().map(|x| x.tick());
            }
            Err(TryRecvError::Disconnected) => {
                debug_print!("rust thread disconnected, quit player thread.");
                break;
            }
        }
    }
}
