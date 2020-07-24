use mpris::{PlayerFinder, Metadata};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use mpris::PlaybackStatus::Playing;

fn main() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    if player.bus_name().to_string() == String::from("org.mpris.MediaPlayer2.spotify") {
        match player.get_playback_status() {
            Ok(result) => if result != Playing {
                create_now_playing_file("今は音楽を再生していないみたいです".to_string());
                return;
            }
            Err(msg) => println!("error: {:#?}", msg)
        }

        let metadata = player.get_metadata().expect("Could not get metadata for player");
        let message = get_now_playing_message(metadata);
        create_now_playing_file(message);
    }
}

fn get_now_playing_message(metadata: Metadata) -> String {
    let mut artists = String::new();

    for artist in metadata.artists().unwrap() {
        artists += artist; // 現状SpotifyはD-Busで一件のアーティストしか渡さないためスペースを開けたりしていないが、将来的に改善が必要になるかもしれない
    }

    let song_title = metadata.title().unwrap();
    return format!("今は{}の{}を聴いているよ！", artists, song_title);
}

fn create_now_playing_file(message: String) {
    let path = Path::new("/tmp/now_playing.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(message.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
