use rodio::{Decoder, OutputStream, Sink};
use rust_embed::RustEmbed;
use std::io;

#[derive(RustEmbed)]
#[folder = "assets\\"]
struct Asset;

fn main() {
    // Grab the file from embedded area.
    let music = match Asset::get("Override.ogg") {
        Some(o) => o,
        None => panic!(),
    };

    // init Rodio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // use Sinks to allow it to wait till end.
    let sink = Sink::try_new(&stream_handle).unwrap();
    let buf_music = Decoder::new_vorbis(io::Cursor::new(music.data.into_owned())).unwrap();

    sink.append(buf_music);
    sink.sleep_until_end();
}
