use symphonia::core::{
    audio::AudioBufferRef,
    codecs::{Decoder, CODEC_TYPE_NULL},
    formats::FormatReader,
    io::{MediaSource, MediaSourceStream},
    probe::Hint,
};

pub struct Audio {
    format: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn Decoder>,
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

    pub fn get_next_sample(&mut self) -> AudioBufferRef {
        let packet = match self.format.next_packet() {
            Ok(packet) => packet,
            Err(e) => panic!("{e}"),
        };

        assert_eq!(self.track_id, packet.track_id());

        let decoded = match self.decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(e) => panic!("{e}"),
        };

        return decoded;
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
        let decoded = audio.get_next_sample();
        let meta = decoded.spec();
        // println!("{:?}", meta)
    }
}
