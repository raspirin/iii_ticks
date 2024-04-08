use symphonia::core::{
    audio::AudioBufferRef,
    codecs::{Decoder, CODEC_TYPE_NULL},
    formats::FormatReader,
    io::{MediaSource, MediaSourceStream},
    probe::Hint,
};

pub struct Audio {
    pub format: Box<dyn FormatReader>,
    pub track_id: u32,
    pub decoder: Box<dyn Decoder>,
}

impl Audio {
    pub fn new(source: Box<dyn MediaSource>) -> Self {
        let mss = MediaSourceStream::new(source, Default::default());

        let mut hint = Hint::new();
        hint.with_extension("ogg");

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &Default::default(), &Default::default())
            .unwrap();

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .unwrap();
        let track_id = track.id;

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &Default::default())
            .unwrap();

        Self {
            format,
            track_id,
            decoder,
        }
    }

}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_make_audio() {
        let file = File::open("../../assets/native/test.ogg").unwrap();
        let mut audio = Audio::new(Box::new(file));
    }
}
