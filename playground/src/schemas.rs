
use std::io::Write;
use std::ops::Deref;
use std::path::Path;


use versebase::table::{Table, TableSchema};
use versebase::index::{TableIndex};
use versebase::datatypes::{Int, Str, DateTime, DataType};
use versebase::datatypes;


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

#[derive(TableSchema, Debug)]
pub struct Songs {
    pub id: Int,
    pub name: Str,
    pub artist_id: Int,
}


#[derive(TableSchema, Debug)]
pub struct Lyrics {
    pub id: Int,
    pub text: Str,
    pub language: Str,
    pub song_id: Int,
}

#[derive(TableSchema, Debug)]
pub struct LikedSongs {
    pub id: Int,
    pub song_id: Int,
    pub user_id: Int,
    pub created_at: DateTime,
}
