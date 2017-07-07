use super::metadata;
use std::collections::HashMap;
use std::path::Path;

extern crate ffmpeg;

/// Metadata for an audio file that has been read through FFMpeg.
pub struct AudioFile {
    metadata_dict: HashMap<String, String>,
}

/// Trait implementation for providing metadata.
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

/// Error encountered when reading an audio file through FFmpeg.
#[derive(Debug)]
pub enum AudioFileReadingError {
    /// No file was found.
    NoFile,
    /// The file could not be read as an audio file.
    NotAudioFile,
    /// Other ffmpeg opening error.
    OpeningError(ffmpeg::Error),
}

impl AudioFile {
    /// Read a file from a given filename.
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