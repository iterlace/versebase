
use std::io::Write;
use std::ops::Deref;
use std::path::Path;


use versebase::table::{Table};
use versebase::index::{TableIndex};
use versebase::datatypes::{Int, Str, DateTime, DataType};
use versebase::datatypes;

use super::schemas::{Songs, Lyrics, Artists, LikedSongs, Users};


pub struct Database {
    pub users: Table<Users>,
    pub songs: Table<Songs>,
    pub lyrics: Table<Lyrics>,
    pub artists: Table<Artists>,
    pub liked_songs: Table<LikedSongs>,
}

impl Database {
    pub fn new() -> Self {
        let users = Table::<Users>::new(
            String::from("users"),
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/users.tbl")),
            Some(TableIndex::new(
                Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/users.idx"))
            ).unwrap()),
        ).unwrap();

        let songs = Table::<Songs>::new(
            String::from("songs"),
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/songs.tbl")),
            Some(TableIndex::new(
                Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/songs.idx"))
            ).unwrap()),
        ).unwrap();

        let lyrics = Table::<Lyrics>::new(
            String::from("lyrics"),
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/lyrics.tbl")),
            Some(TableIndex::new(
                Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/lyrics.idx"))
            ).unwrap()),
        ).unwrap();

        let artists = Table::<Artists>::new(
            String::from("artists"),
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/artists.tbl")),
            Some(TableIndex::new(
                Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/artists.idx"))
            ).unwrap()),
        ).unwrap();

        let liked_songs = Table::<LikedSongs>::new(
            String::from("liked_songs"),
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/liked_songs.tbl")),
            Some(TableIndex::new(
                Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/liked_songs.idx"))
            ).unwrap()),
        ).unwrap();

        Self {
            users,
            songs,
            lyrics,
            artists,
            liked_songs,
        }
    }
}
