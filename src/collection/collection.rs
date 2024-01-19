use crate::{
    song::{Genre, Song},
    user::PlaylistGenerationType,
};

#[derive(Debug, Clone)]
pub enum FilterOption {
    StringQuery(String),
    GenreQuery(Genre),
}
#[derive(Debug, Clone)]
pub struct SongCollection {
    title: String,
    pub songs: Vec<Song>,
}
pub trait SongCollectionInterface {
    fn new(title: String) -> SongCollection;
    fn get_songs(&self) -> &Vec<Song>;
    fn add_song(&mut self, song: Song);
    fn get_song(&self, index: usize) -> Option<Song>;
    fn search(&self, query: String) -> Vec<Song>;
    fn filter(&self, query: PlaylistGenerationType) -> Vec<Song>;
}
impl SongCollectionInterface for SongCollection {
    fn new(title: String) -> SongCollection {
        let songs: Vec<Song> = vec![];
        SongCollection { title, songs }
    }
    fn get_songs(&self) -> &Vec<Song> {
        &self.songs
    }
    fn get_song(&self, index: usize) -> Option<Song> {
        self.songs.get(index).cloned()
    }
    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }
    // by song title or artist
    fn search(&self, query: String) -> Vec<Song> {
        let songs: Vec<Song> = self.songs.clone();
        songs
            .into_iter()
            .filter(|song| {
                let song_words: Vec<&str> = song.title.as_str().split_whitespace().collect();
                let artist_words: Vec<&str> = song.artist.as_str().split_whitespace().collect();
                let query_words: Vec<&str> = query.as_str().split_whitespace().collect();
                // find song or artist with every query word.
                query_words.iter().all(|&q_word| {
                    let song_matchs = song_words
                        .iter()
                        .any(|&s_word| q_word.to_ascii_lowercase() == s_word.to_ascii_lowercase());
                    if !song_matchs {
                        artist_words.iter().any(|&a_word| {
                            q_word.to_ascii_lowercase() == a_word.to_ascii_lowercase()
                        })
                    } else {
                        return song_matchs;
                    }
                })
            })
            .collect()
    }

    fn filter(&self, query: PlaylistGenerationType) -> Vec<Song> {
        self.songs
            .iter()
            .filter(|&song| match &query {
                PlaylistGenerationType::SimilarArtist(query_str) => &song.artist == query_str,
                PlaylistGenerationType::Genre(genre_query) => &song.genre == genre_query,
            })
            .cloned()
            .collect()
    }
}
