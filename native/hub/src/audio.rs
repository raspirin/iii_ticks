use symphonia::core::{
    codecs::{Decoder, CODEC_TYPE_NULL},
    formats::{FormatReader, SeekMode, SeekTo},
    io::{MediaSource, MediaSourceStream},
    probe::Hint,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Probe error: {0}")]
    ProbeError(#[source] symphonia::core::errors::Error),
    #[error("Fount no playable track.")]
    TrackNotFound,
    #[error("Decoder error: {0}")]
    DecoderError(#[source] symphonia::core::errors::Error),
    #[error("Unable to reset to the starting point.")]
    SeekError(#[source] symphonia::core::errors::Error),
}

pub struct Audio {
    pub format: Box<dyn FormatReader>,
    pub track_id: u32,
    pub decoder: Box<dyn Decoder>,
}

impl Audio {
    pub fn try_new(source: Box<dyn MediaSource>) -> Result<Self, AudioError> {
        let mss = MediaSourceStream::new(source, Default::default());

        let mut hint = Hint::new();
        hint.with_extension("ogg");

        let probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &Default::default(),
            &Default::default(),
        ) {
            Ok(p) => p,
            Err(e) => return Err(AudioError::DecoderError(e)),
        };

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or(AudioError::TrackNotFound)?;
        let track_id = track.id;

        let decoder =
            match symphonia::default::get_codecs().make(&track.codec_params, &Default::default()) {
                Ok(d) => d,
                Err(e) => return Err(AudioError::DecoderError(e)),
            };

        Ok(Self {
            format,
            track_id,
            decoder,
        })
    }

    pub fn reset(&mut self) -> Result<(), AudioError> {
        if let Err(e) = self.format.seek(
            SeekMode::Accurate,
            SeekTo::TimeStamp {
                ts: 0,
                track_id: self.track_id,
            },
        ) {
            return Err(AudioError::SeekError(e));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_make_audio() {
        let file = File::open("../../assets/native/test.ogg").unwrap();
        let mut audio = Audio::try_new(Box::new(file));
    }
}
