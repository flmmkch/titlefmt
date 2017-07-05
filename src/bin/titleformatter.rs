extern crate ffmpeg;
extern crate titleformat;
use std::env;
use titleformat::Formatter;
use titleformat::ffmpeg_audio::*;

fn main() {
	// first: initialize ffmpeg
	ffmpeg::init().unwrap();
	// read arguments
	let args: Vec<String> = env::args().collect();
	let formatter = Formatter::new();
	let parser = formatter.parser();
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
		let expression = parser.parse(expression_string.as_str()).unwrap();
		// read the various files and apply it to them
		for filename in filenames.iter() {
			let audio_file_result = AudioFile::read_file(filename);
			match audio_file_result {
				Ok(audio_file) => {
					let line = expression.apply(&audio_file);
					println!("{}", line);
				},
				Err(_) => continue,
			};
		}
	}
}
