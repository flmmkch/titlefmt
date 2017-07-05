use super::metadata;
use std::collections::HashMap;
use std::path::Path;

extern crate ffmpeg;

pub struct AudioFile {
    pub metadata_dict: HashMap<String, String>,
}

impl metadata::Provider for AudioFile {
    fn tag_value(&self, key: &str) -> Option<String> {
        let entry = self.metadata_dict.get(key);
        if let Some(value) = entry {
            Some(value.to_owned())
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
pub enum AudioFileReadingError {
    NoFile,
    NotAudioFile,
    OpeningError(ffmpeg::Error),
}

impl AudioFile {
    pub fn read_file(filename: &str) -> Result<AudioFile, AudioFileReadingError> {
        let path = Path::new(filename);
        if !path.exists() {
            return Err(AudioFileReadingError::NoFile);
        }
        match ffmpeg::format::input(&filename) {
            Ok(context) => {
                let mut metadata_dict: HashMap<String, String> = HashMap::new();
                // check that this is indeed an audio file
                {
                    let mut is_audio_stream = false;
                    for stream in context.streams() {
                        let codec = stream.codec();
                        if codec.medium() == ffmpeg::media::Type::Audio {
                            is_audio_stream = true;
                        }
                    }
                    if !is_audio_stream {
                        return Err(AudioFileReadingError::NotAudioFile);
                    }
                }
                for (k, v) in context.metadata().iter() {
                    metadata_dict.insert(k.to_lowercase(), v.to_owned());
                }
                let audio_file = AudioFile {
                    metadata_dict,
                };
                Ok(audio_file)
            }

            Err(error) =>
                Err(AudioFileReadingError::OpeningError(error))
	    }
    }
}