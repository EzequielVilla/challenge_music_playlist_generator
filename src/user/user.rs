use dialoguer::{Input, Select};
use uuid::Uuid;

use crate::{
    collection::{SongCollection, SongCollectionInterface},
    playlist::{Playlist, PlaylistInterface},
    song::{Genre, Song, SongInterface},
    GREEN, RESET,
};

enum UserFunctions {
    CreateSong,
    FindSong,
    CreatePlaylist,
    FillPlaylist,
    AddSongToPlaylist,
    SeeMyPlaylists,
    SeeAllSongsInCollection,
    SeeSongsInSelectedPlaylist,
}
pub enum PlaylistGenerationType {
    Genre(Genre),
    SimilarArtist(String),
}

pub struct User {
    id: Uuid,
    playlists: Vec<Playlist>,
    collection: SongCollection,
}
pub trait UserInterface {
    fn new(collection: SongCollection) -> User;
    fn user_functions(&mut self);
    fn create_song(&mut self);
    fn find_song(&mut self);
    fn create_playlist(&mut self);
    fn fill_playlist_metrics_or_recommended(&mut self);
    fn add_one_song_to_playlist(&mut self);
    fn print_all_playlists(&mut self);
    fn print_all_songs(&mut self);
    fn print_songs_in_playlist(&mut self);
}

impl UserInterface for User {
    fn new(collection: SongCollection) -> User {
        User {
            id: Uuid::new_v4(),
            playlists: vec![],
            collection,
        }
    }
    fn user_functions(&mut self) {
        let user_functions = &[
            "Create song",
            "Find song",
            "Create playlist",
            "Fill Playlist by artist or genre",
            "Add one song to playlist",
            "See my playlists",
            "See all songs in collection",
            "See songs in one playlist",
        ];
        let option_selection = Select::new()
            .with_prompt("Select an action")
            .items(user_functions)
            .interact()
            .unwrap();
        let option = match option_selection {
            0 => UserFunctions::CreateSong,
            1 => UserFunctions::FindSong,
            2 => UserFunctions::CreatePlaylist,
            3 => UserFunctions::FillPlaylist,
            4 => UserFunctions::AddSongToPlaylist,
            5 => UserFunctions::SeeMyPlaylists,
            6 => UserFunctions::SeeAllSongsInCollection,
            7 => UserFunctions::SeeSongsInSelectedPlaylist,
            _ => panic!("Invalid selection"),
        };
        match option {
            UserFunctions::CreateSong => self.create_song(),
            UserFunctions::FindSong => self.find_song(),
            UserFunctions::CreatePlaylist => self.create_playlist(),
            UserFunctions::FillPlaylist => self.fill_playlist_metrics_or_recommended(),
            UserFunctions::AddSongToPlaylist => self.add_one_song_to_playlist(),
            UserFunctions::SeeMyPlaylists => self.print_all_playlists(),
            UserFunctions::SeeAllSongsInCollection => self.print_all_songs(),
            UserFunctions::SeeSongsInSelectedPlaylist => self.print_songs_in_playlist(),
        }

        println!(
            "{}------------------------------------------------------{}",
            GREEN, RESET
        );
        println!(
            "{}------------------------------------------------------{}",
            GREEN, RESET
        );
        self.user_functions();
    }
    fn create_song(&mut self) {
        let genre_options: &[&str; 4] = &["Rock", "Electronic", "Pop", "Jazz"];
        let title: String = Input::new()
            .with_prompt("Enter the title")
            .interact_text()
            .unwrap();
        let artist: String = Input::new()
            .with_prompt("Enter the artist")
            .interact_text()
            .unwrap();
        let genre_selection: usize = Select::new()
            .with_prompt("Select the genre")
            .items(genre_options)
            .interact()
            .unwrap();
        let genre: Genre = match genre_selection {
            0 => Genre::Rock,
            1 => Genre::Electronic,
            2 => Genre::Pop,
            3 => Genre::Jazz,
            _ => panic!("Invalid genre selection"),
        };
        let duration: f32 = Input::new()
            .with_prompt("Enter the duration")
            .interact_text()
            .unwrap();
        let release_year: i32 = Input::new()
            .with_prompt("Enter the release year")
            .interact_text()
            .unwrap();
        let new_song: Song = Song::new(title, artist, genre, duration, release_year);
        self.collection.add_song(new_song); //todo Change to use reference and not clone.
    }
    fn find_song(&mut self) {
        let query = Input::new()
            .with_prompt("Search a song by title or artist")
            .interact_text()
            .unwrap();
        let songs: Vec<Song> = self.collection.search(query);
        if songs.len() > 0 {
            for song in songs {
                println!("-----------SONG-----------");
                println!("Title: {:?}.", song.title);
                println!("Artist: {:?}.", song.artist);
                println!("Genre: {:?}.", song.genre);
                println!("Duration: {:?}.", song.duration);
            }
        } else {
            let ask_again_selected = Select::new()
                .with_prompt("Want to search again?")
                .items(&["Yes", "No"])
                .interact()
                .unwrap();
            if ask_again_selected == 0 {
                self.find_song()
            }
        }
    }
    fn create_playlist(&mut self) {
        let title = Input::new()
            .with_prompt("Insert playlist title")
            .interact_text()
            .unwrap();
        let new_playlist: Playlist = Playlist::new(title);
        self.playlists.push(new_playlist);
    }
    fn fill_playlist_metrics_or_recommended(&mut self) {
        let fill_options = &["Genre", "Similar artist"];
        let all_playlist_titles: Vec<String> = self
            .playlists
            .iter()
            .map(|playlist| playlist.title.clone())
            .collect();
        let playlist_selected = Select::new()
            .with_prompt("Select which playlist do you want to fill")
            .items(&all_playlist_titles)
            .interact()
            .unwrap();
        let fill_selection = Select::new()
            .with_prompt("Select how to fill the playlist")
            .items(fill_options)
            .interact()
            .unwrap();
        let fill_option = match fill_selection {
            0 => {
                let genre_options: &[&str; 4] = &["Rock", "Electronic", "Pop", "Jazz"];
                let genre_selection: usize = Select::new()
                    .with_prompt("Select the genre")
                    .items(genre_options)
                    .interact()
                    .unwrap();
                let genre: Genre = match genre_selection {
                    0 => Genre::Rock,
                    1 => Genre::Electronic,
                    2 => Genre::Pop,
                    3 => Genre::Jazz,
                    _ => panic!("Invalid genre selection"),
                };
                PlaylistGenerationType::Genre(genre)
            }
            1 => {
                let artist = Input::new()
                    .with_prompt("Enter the similar artist")
                    .interact_text()
                    .unwrap();
                PlaylistGenerationType::SimilarArtist(artist)
            }
            _ => panic!("Invalid selection"),
        };
        self.playlists[playlist_selected].fill_by_option(fill_option, &mut self.collection);
    }
    fn add_one_song_to_playlist(&mut self) {
        let query = Input::new()
            .with_prompt("Search a song to add. Insert the title")
            .interact_text()
            .unwrap();
        let query_result: Vec<Song> = self.collection.search(query);
        let titles: Vec<String> = query_result
            .iter()
            .map(|song: &Song| song.title.clone())
            .collect();
        if titles.len() > 0 {
            let title_selected = Select::new()
                .with_prompt("Select an option with the results")
                .items(&titles)
                .interact()
                .unwrap();
            let song_selected = query_result[title_selected].clone();
            self.print_all_playlists();
            let all_playlist_titles: Vec<String> = self
                .playlists
                .iter()
                .map(|playlist| playlist.title.clone())
                .collect();
            let playlist_selected = Select::new()
                .with_prompt("Select which playlist do you want to add the song")
                .items(&all_playlist_titles)
                .interact()
                .unwrap();
            self.playlists[playlist_selected].add_song_selected_by_user(song_selected)
        } else {
            let ask_again_selected = Select::new()
                .with_prompt("Want to search again?")
                .items(&["Yes", "No"])
                .interact()
                .unwrap();
            match ask_again_selected {
                0 => self.add_one_song_to_playlist(),
                1 => self.add_one_song_to_playlist(),
                _ => panic!("Invalid option"),
            }
        }
    }
    fn print_all_playlists(&mut self) {
        for playlist in &self.playlists {
            println!("Playlist title: {:?}", playlist.title);
        }
    }
    fn print_all_songs(&mut self) {
        let actual_songs = self.collection.get_songs();
        for song in actual_songs {
            println!("-----------SONG-----------");
            println!("Title: {:?}.", song.title);
            println!("Artist: {:?}.", song.artist);
            println!("Genre: {:?}.", song.genre);
            println!("Duration: {:?}.", song.duration);
        }
    }
    fn print_songs_in_playlist(&mut self) {
        let all_playlist_titles: Vec<String> = self
            .playlists
            .iter()
            .map(|playlist| playlist.title.clone())
            .collect();
        if all_playlist_titles.len() > 0 {
            let playlist_selected = Select::new()
                .with_prompt("Select which playlist do you want to add the song")
                .items(&all_playlist_titles)
                .interact()
                .unwrap();
            let playlist_songs = self.playlists[playlist_selected].get_songs();
            for song in playlist_songs {
                println!("-----------SONG-----------");
                println!("Title: {:?}.", song.title);
                println!("Artist: {:?}.", song.artist);
                println!("Genre: {:?}.", song.genre);
                println!("Duration: {:?}.", song.duration);
            }
        } else {
            println!("No playlist created, create one first.");
        }
    }
}
