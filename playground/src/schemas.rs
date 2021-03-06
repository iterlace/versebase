use std::any::Any;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;


use versebase::error;
use versebase::table::{Table, TableSchema};
use versebase::index::{TableIndex};
use versebase::datatypes::{Int, Str, DateTime, DataType, DType};
use versebase::datatypes;
use super::db::Database;


#[derive(TableSchema, Debug)]
pub struct Users {
    pub id: Int,
    pub email: Str,
    pub password: Str,
    pub salt: Str,
    pub language: Str,
    pub last_login: DateTime,
}

#[derive(TableSchema, Debug)]
pub struct Artists {
    pub id: Int,
    pub name: Str,
}

impl Artists {
    pub fn get_songs(&self, db: &mut Database) -> Vec<Songs> {
        db.songs.get(1);
        vec![]
    }
}

#[derive(TableSchema, Debug)]
pub struct Songs {
    pub id: Int,
    pub name: Str,
    pub artist_id: Int,
}

impl Songs {
    pub fn get_artist(&self, db: &mut Database) -> Result<Artists, versebase::error::Error> {
        db.artists.get(self.artist_id.get())
    }
}


#[derive(TableSchema, Debug)]
pub struct Lyrics {
    pub id: Int,
    pub text: Str,
    pub language: Str,
    pub song_id: Int,
}

impl Lyrics {
    pub fn get_song(&self, db: &mut Database) -> Result<Songs, versebase::error::Error> {
        db.songs.get(self.song_id.get())
    }
}

#[derive(TableSchema, Debug)]
pub struct LikedSongs {
    pub id: Int,
    pub song_id: Int,
    pub user_id: Int,
    pub created_at: DateTime,
}

impl LikedSongs {
    pub fn get_user(&self, db: &mut Database) -> Result<Users, versebase::error::Error> {
        db.users.get(self.user_id.get())
    }
    pub fn get_song(&self, db: &mut Database) -> Result<Songs, versebase::error::Error> {
        db.songs.get(self.song_id.get())
    }
}
