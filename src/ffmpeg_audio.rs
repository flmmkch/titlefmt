
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
                /*
                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
                    println!("Best video stream index: {}", stream.index());
                }
            
                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
                    println!("Best audio stream index: {}", stream.index());
                }
            
                if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
                    println!("Best subtitle stream index: {}", stream.index());
                }

                println!("duration (seconds): {:.2}", context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64);
                
                for stream in context.streams() {
                    println!("stream index {}:", stream.index());
                    println!("\ttime_base: {}", stream.time_base());
                    println!("\tstart_time: {}", stream.start_time());
                    println!("\tduration (stream timebase): {}", stream.duration());
                    println!("\tduration (seconds): {:.2}", stream.duration() as f64 * f64::from(stream.time_base()));
                    println!("\tframes: {}", stream.frames());
                    println!("\tdisposition: {:?}", stream.disposition());
                    println!("\tdiscard: {:?}", stream.discard());
                    println!("\trate: {}", stream.rate());
            
                    let codec = stream.codec();
                    println!("\tmedium: {:?}", codec.medium());
                    println!("\tid: {:?}", codec.id());
            
                    if codec.medium() == ffmpeg::media::Type::Video {
                        if let Ok(video) = codec.decoder().video() {
                            println!("\tbit_rate: {}", video.bit_rate());
                            println!("\tmax_rate: {}", video.max_bit_rate());
                            println!("\tdelay: {}", video.delay());
                            println!("\tvideo.width: {}", video.width());
                            println!("\tvideo.height: {}", video.height());
                            println!("\tvideo.format: {:?}", video.format());
                            println!("\tvideo.has_b_frames: {}", video.has_b_frames());
                            println!("\tvideo.aspect_ratio: {}", video.aspect_ratio());
                            println!("\tvideo.color_space: {:?}", video.color_space());
                            println!("\tvideo.color_range: {:?}", video.color_range());
                            println!("\tvideo.color_primaries: {:?}", video.color_primaries());
                            println!("\tvideo.color_transfer_characteristic: {:?}", video.color_transfer_characteristic());
                            println!("\tvideo.chroma_location: {:?}", video.chroma_location());
                            println!("\tvideo.references: {}", video.references());
                            println!("\tvideo.intra_dc_precision: {}", video.intra_dc_precision());
                        }
                    }
                    else
                    if codec.medium() == ffmpeg::media::Type::Audio {
                        if let Ok(audio) = codec.decoder().audio() {
                            println!("\tbit_rate: {}", audio.bit_rate());
                            println!("\tmax_rate: {}", audio.max_bit_rate());
                            println!("\tdelay: {}", audio.delay());
                            println!("\taudio.rate: {}", audio.rate());
                            println!("\taudio.channels: {}", audio.channels());
                            println!("\taudio.format: {:?}", audio.format());
                            println!("\taudio.frames: {}", audio.frames());
                            println!("\taudio.align: {}", audio.align());
                            println!("\taudio.channel_layout: {:?}", audio.channel_layout());
                            println!("\taudio.frame_start: {:?}", audio.frame_start());
                        }
                    }
                }*/
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