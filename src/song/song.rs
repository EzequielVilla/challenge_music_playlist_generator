use uuid::Uuid;
#[derive(Debug, Clone, PartialEq)]
pub enum Genre {
    Rock,
    Electronic,
    Pop,
    Jazz,
}

#[derive(Debug, Clone)]

pub struct Song {
    id: Uuid,
    pub title: String,
    pub artist: String,
    pub genre: Genre,
    pub duration: f32, // convert to number time
    pub release_year: i32,
}

pub trait SongInterface {
    fn new(title: String, artist: String, genre: Genre, duration: f32, release_year: i32) -> Song;
}

impl SongInterface for Song {
    fn new(title: String, artist: String, genre: Genre, duration: f32, release_year: i32) -> Song {
        let new_song = Song {
            id: Uuid::new_v4(),
            title,
            artist,
            genre,
            duration,
            release_year,
        };
        new_song
    }
}
