pub mod collection;
pub mod playlist;
pub mod song;
pub mod user;
use crate::collection::{SongCollection, SongCollectionInterface};
use crate::song::{Song, SongInterface};
use crate::user::{User, UserInterface};
// Color for the line which separe user interactions
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
fn main() {
    let mut new_collection: SongCollection = SongCollection::new("Coleccion 1".to_string());
    push_mock_songs(&mut new_collection);
    let mut user = User::new(new_collection.clone());
    user.user_functions();
}

fn push_mock_songs(new_collection: &mut SongCollection) {
    let title = "Bag Pack".to_string();
    let artist = "Clem Beatz".to_string();
    let genre = song::Genre::Rock;
    let duration = 10.30;
    let release_year = 1993;
    let new_song = Song::new(title, artist, genre, duration, release_year);
    new_collection.add_song(new_song);
    let title_2 = "Playground Games".to_string();
    let artist_2 = "TM Juke, Alice Russell".to_string();
    let genre_2 = song::Genre::Jazz;
    let duration_2 = 10.33;
    let release_year_2 = 1982;
    let new_song_2 = Song::new(title_2, artist_2, genre_2, duration_2, release_year_2);
    new_collection.add_song(new_song_2);
    let title_3 = "Daydream".to_string();
    let artist_3 = "Clem Beatz".to_string();
    let genre_3 = song::Genre::Jazz;
    let duration_3 = 10.33;
    let release_year_3 = 1983;
    let new_song_3 = Song::new(title_3, artist_3, genre_3, duration_3, release_year_3);
    new_collection.add_song(new_song_3);
    let title_4 = "Polyfizzal drizzal".to_string();
    let artist_4 = "Desmond Cheese".to_string();
    let genre_4 = song::Genre::Jazz;
    let duration_4 = 10.33;
    let release_year_4 = 1984;
    let new_song_4 = Song::new(title_4, artist_4, genre_4, duration_4, release_year_4);
    new_collection.add_song(new_song_4);
}
