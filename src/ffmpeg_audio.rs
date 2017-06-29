
use super::metadata;
use std::collections::HashMap;

extern crate ffmpeg;

pub struct AudioFile {
    pub metadata_dict: HashMap<String, String>,
}

impl metadata::MetadataObject for AudioFile {
    fn read_tag(&self, key: &str) -> Option<String> {
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
    OpeningError(String, ffmpeg::Error),
}

impl AudioFile {
    pub fn read_file(filename: &str) -> Result<AudioFile, AudioFileReadingError> {
        match ffmpeg::format::input(&filename) {
            Ok(context) => {
                let mut metadata_dict: HashMap<String, String> = HashMap::new();
                for (k, v) in context.metadata().iter() {
                    metadata_dict.insert(k.to_lowercase(), v.to_owned());
                }
                let audio_file = AudioFile {
                    metadata_dict,
                };
                Ok(audio_file)
            }

            Err(error) =>
                Err(AudioFileReadingError::OpeningError(filename.to_owned(), error))
	    }
    }
}