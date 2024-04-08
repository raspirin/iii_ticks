use std::fs::File;
use crate::{audio::Audio, output::Output};

pub struct Player {
    audio: Audio,
    output: Option<Output>,
}

impl Player {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let audio = Audio::new(Box::new(file));

        Self {
            audio,
            output: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_player() {
        let player = Player::new("../../assets/native/test.ogg");
    }
}