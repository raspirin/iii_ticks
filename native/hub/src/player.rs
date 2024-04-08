use crate::{audio::Audio, output::{AudioOutput, AudioOutputFactory, Output}};
use std::fs::File;

pub struct Player {
    audio: Audio,
    output: Option<Box<dyn AudioOutput>>,
}

impl Player {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let audio = Audio::new(Box::new(file));

        Self {
            audio,
            output: None,
        }
    }

    pub fn play(&mut self) {
        loop {
            let packet = self.audio.format.next_packet().unwrap();
            if packet.track_id() != self.audio.track_id {
                continue;
            }

            match self.audio.decoder.decode(&packet) {
                Ok(decoded) => {
                    if self.output.is_none() {
                        let spec = *decoded.spec();
                        let duration = decoded.capacity() as u64;
                        self.output.replace(AudioOutputFactory::open(&spec, duration));
                    }

                    if let Some(output) = &mut self.output {
                        output.write(decoded);
                    }
                },
                Err(e) => panic!("decoded error: {e}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_player() {
        let mut player = Player::new("../../assets/native/test.ogg");
        player.play();
    }
}
