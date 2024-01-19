use crate::{
    collection::{SongCollection, SongCollectionInterface},
    song::{Genre, Song},
    user::PlaylistGenerationType,
};
use uuid::Uuid;
#[derive(Debug)]
pub struct Playlist {
    id: Uuid,
    pub title: String,
    songs: Vec<Song>,
}

pub trait PlaylistInterface {
    fn new(title: String) -> Playlist;
    fn get_songs(&self) -> &Vec<Song>;
    fn fill_by_option(&mut self, option: PlaylistGenerationType, collection: &mut SongCollection);
    fn add_song_selected_by_user(&mut self, song: Song);
}
impl PlaylistInterface for Playlist {
    fn new(title: String) -> Playlist {
        Playlist {
            id: Uuid::new_v4(),
            title,
            songs: vec![],
        }
    }
    fn fill_by_option(&mut self, option: PlaylistGenerationType, collection: &mut SongCollection) {
        let songs = collection.filter(option);
        for song in songs {
            self.songs.push(song)
        }
    }

    fn add_song_selected_by_user(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn get_songs(&self) -> &Vec<Song> {
        &self.songs
    }
}
