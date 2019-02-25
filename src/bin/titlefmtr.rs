extern crate id3;
extern crate metaflac;
extern crate titlefmt;
use std::env;
use std::path::{Path, PathBuf};
use titlefmt::Formatter;

fn main() {
    // read arguments
    let args: Vec<String> = env::args().collect();
    let formatter = Formatter::new();
    let parser = formatter.parser();
    let mut display_help = true;
    if args.len() > 2 {
        // first argument: expression
        let expression_string = String::from(args[1].to_owned());
        // second argument and subsequent ones: filenames
        let filenames = {
            let mut filenames_builder: Vec<String> = Vec::new();
            for i in 2..args.len() {
                filenames_builder.push(args[i].to_owned());
            }
            filenames_builder
        };
        // parse the expression
        match parser.parse(expression_string.as_str()) {
            Ok(expression) => {
                display_help = false;
                // read the various files and apply it to them
                for filename in filenames.iter() {
                    let audio_file_result = MetadataFile::read_file(filename);
                    match audio_file_result {
                        Ok(audio_file) => {
                            let line = expression.apply(&audio_file);
                            println!("{}", line);
                        }
                        Err(_) => continue,
                    };
                }
            }
            Err(error) => println!("Invalid expression: {:?}", error),
        }
    }
    if display_help {
        println!("Usage:");
        println!("{} [expression] [filenames...]", &args[0]);
        println!("where [expression] is a Title Formatting expression, such as \"%tracknumber%. [%artist% - ]%title%\"");
    }
}

enum MetadataFile {
    FlacMetadata(metaflac::Tag),
    Id3Metadata(id3::Tag),
}

enum MetadataError {
    FlacErr(metaflac::Error),
    Id3Err(id3::Error),
    BadFormat(PathBuf),
}

impl MetadataFile {
    fn read_file<P: AsRef<Path>>(as_path: P) -> Result<MetadataFile, MetadataError> {
        let path: &Path = as_path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str());
        match extension {
            Some("flac") => match metaflac::Tag::read_from_path(path) {
                Ok(flac_metadata) => Ok(MetadataFile::FlacMetadata(flac_metadata)),
                Err(flac_error) => Err(MetadataError::FlacErr(flac_error)),
            },
            Some("mp3") => match id3::Tag::read_from_path(path) {
                Ok(id3_metadata) => Ok(MetadataFile::Id3Metadata(id3_metadata)),
                Err(id3_err) => Err(MetadataError::Id3Err(id3_err)),
            },
            _ => Err(MetadataError::BadFormat(path.to_owned())),
        }
    }
}

impl titlefmt::metadata::Provider for MetadataFile {
    fn tag_value(&self, tag_name: &str) -> Option<String> {
        match self {
            &MetadataFile::FlacMetadata(ref flac_metadata) => flac_metadata
                .get_vorbis(tag_name)
                .and_then(|v| v.first())
                .cloned(),
            &MetadataFile::Id3Metadata(ref id3_metadata) => id3_metadata
                .get(tag_name)
                .and_then(|frame| frame.content().text())
                .map(String::from),
        }
    }
}
