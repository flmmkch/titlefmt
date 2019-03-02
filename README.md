# titlefmt

**titlefmt** is a title formatting library for media files written in Rust. The aim of the project is to be conformant to the foobar2000 formatting syntax.

For more reference about the Foobar syntax, please refer to http://wiki.hydrogenaud.io/index.php?title=Foobar2000:Title_Formatting_Reference

To try out the formatting features, an example is included with the `titlefmtr` executable binary. Use a formatting expression as first argument, such as `"%track%. %title%"`, and the audio files you want to apply the title formatting to as subsequent arguments.

Example: `$ cargo run --release --bin titlefmtr "%track%. [%artist% - ] %title%" test01.flac test02.mp3 music/**/*.flac`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
titlefmt = "0.4"
```

[The library documentation is available on docs.rs](https://docs.rs/titlefmt/).

## Currently implemented functionalities
* Field references (tags), such as `%artist%` or `%title%`, with values provided using the `Provider` trait.
* Conditional selection within square brackets `[ ]` for sub-expressions that only appear if they contain a valid tag. 
* Advanced tag handling (expanding `%tracknumber%` with a leading zero if it's only one digit, aliasing `%album artist%` and `%artist%`, etc.).
* Using functions with `$function(arg1, arg2, arg3)`. A [list of default functions with implementations provided by the crate](FUNCTIONS.md) is available. It is possible to add new functions to the title formatting script.

## Currently unimplemented functionalities
 * Variables
 
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
