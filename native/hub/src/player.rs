use thiserror::Error;

use crate::{
    audio::{Audio, AudioError},
    output::{self, AudioOutput, AudioOutputError},
};
use std::{fs::File, io};

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("{0}")]
    AudioError(#[from] AudioError),
    #[error("{0}")]
    OutputError(#[from] AudioOutputError),
    #[error("Error when opening audio source: {0}")]
    OpenFileError(#[from] io::Error),
    #[error("Error when getting next packet: {0}")]
    PacketError(#[source] symphonia::core::errors::Error),
    #[error("Error when decoding: {0}")]
    DecodeError(#[source] symphonia::core::errors::Error),
    #[error("EOF")]
    EOF,
}

pub struct Player {
    audio: Audio,
    output: Option<Box<dyn AudioOutput>>,
}

impl Player {
    pub fn new(path: &str) -> Result<Self, PlayerError> {
        let file = File::open(path)?;
        let audio = Audio::try_new(Box::new(file))?;

        Ok(Self {
            audio,
            output: None,
        })
    }

    pub fn tick(&mut self) -> Result<(), PlayerError> {
        let packet = match self.audio.format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::ResetRequired) => unimplemented!(),
            Err(symphonia::core::errors::Error::IoError(e)) => {
                if let io::ErrorKind::UnexpectedEof = e.kind() {
                    return Err(PlayerError::EOF);
                } else {
                    return Err(PlayerError::PacketError(
                        symphonia::core::errors::Error::IoError(e),
                    ));
                }
            }
            Err(e) => return Err(PlayerError::PacketError(e)),
        };
        if packet.track_id() != self.audio.track_id {
            return Ok(());
        }

        match self.audio.decoder.decode(&packet) {
            Ok(decoded) => {
                if self.output.is_none() {
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as u64;
                    self.output.replace(output::try_open(&spec, duration)?);
                }

                if let Some(output) = &mut self.output {
                    output.write(decoded);
                }
            }
            Err(e) => return Err(PlayerError::DecodeError(e)),
        }

        Ok(())
    }

    pub fn play(&mut self) -> Result<(), PlayerError> {
        loop {
            match self.tick() {
                Ok(()) => continue,
                Err(PlayerError::EOF) => break Ok(()),
                e => break e,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_player() {
        let mut player = Player::new("../../assets/native/test.ogg").unwrap();
        player.play().unwrap();
    }

    #[test]
    fn test_player_tick() {
        let mut player = Player::new("../../assets/native/test.ogg").unwrap();
        player.tick().unwrap();
    }
}
