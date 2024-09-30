mod aggregator;
use aggregator::{mix_traits, notify, notify2, Song, Video};

fn main() {
    let song = Song {
        title: String::from("August"),
        artist: String::from("Taylor Swift"),
        length: 150,
    };

    let video = Video {
        title: String::from("August"),
        length: 150,
        lyrics: String::from("Taylor swift and Jack Antonoff"),
    };
    notify(&song);
    notify(&video);
    notify2(&video);
    let song_len = mix_traits(&song);
    println!("{song_len}");
}
