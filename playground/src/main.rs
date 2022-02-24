
use std::io::Write;
use std::ops::Deref;
use std::path::Path;


use versebase::table::{Table, TableSchema};
use versebase::index::{TableIndex};
use versebase::datatypes::{Int, Str, DateTime, DataType, DType};
use versebase::datatypes;

use playground::schemas::*;
use playground::db::Database;
use playground::view::Playground;

fn main() {
    let mut app = Playground::new();

    app.run();

    return;

    let mut db = Database::new();

    let artist1_slayer = Artists {
        id: Int::new(1),
        name: Str::new("Slayer".into()),
    };

    let artist2_kasabian = Artists {
        id: Int::new(2),
        name: Str::new("Kasabian".into()),
    };

    let s1 = Songs {
        id: Int::new(1),
        name: Str::new("Seasons In The Abyss".into()),
        artist_id: Int::new(artist1_slayer.id.get())
    };

    let s2 = Songs {
        id: Int::new(2),
        name: Str::new("Underdog".into()),
        artist_id: Int::new(artist2_kasabian.id.get())
    };

    let s3 = Songs {
        id: Int::new(3),
        name: Str::new("Club foot".into()),
        artist_id: Int::new(artist2_kasabian.id.get())
    };

    // db.artists.create(artist1_slayer);
    // db.artists.create(artist2_kasabian);
    //
    // db.songs.create(s1);
    // db.songs.create(s2);
    // db.songs.create(s3);

    // let a = songs.create(s1).unwrap_or_else(|e| panic!("Error creating song1, {:?}", e));
    // let b = songs.create(s2).unwrap();
    // let c = songs.create(s3).unwrap();

    // println!(
    //     "{:?}\n{:?}\n",
    //     db.artists.get(1),
    //     // db.artists.get(1).unwrap().get("name".into()).unwrap(),
    //     db.artists.get(2).unwrap().get("name".to_string()).unwrap(),
    // );

    println!(
        "{:?}\n",
        db.songs.select([("artist_id".to_string(), DType::Int(Int::new(2)))].into()),
    );

    // println!(
    //     "{:?}\n{:?}\n{:?}\n{:?}\n",
    //     db.songs.get(1),
    //     db.songs.get(2),
    //     db.songs.get(3),
    //     db.songs.get(4)
    // );

    println!(
        "{:?}",
        db.songs.get(1).unwrap().get_artist(&mut db)
    );

    // let mut file = std::fs::File::open("/home/a/CLionProjects/versebase_playground/data/a.tbl").unwrap();
    //
    // // We now write at the offset 10.
    // file.write(&[0u8, 5u8, 1u8, 2u8]).unwrap();


    // let raw = Vec::<(String, Box<[u8]>)>::from([
    //     ("id".to_string(), Box::<[u8]>::from([1, 0, 0, 0])),
    //     ("name".to_string(), Box::<[u8]>::from([83, 101, 97, 115, 111, 110, 32, 105, 110, 32, 65, 98, 121, 115, 115])),
    //     ("posted_at".to_string(), Box::<[u8]>::from([247, 87, 252, 144, 10, 91, 211, 22])),
    // ]);
    // let song: Songs = Songs::from_(raw);
    //
    // println!("id={}; name={}; posted_at={}", song.id, song.name, song.posted_at);
    // println!("{:?}", Songs::fields())

    // for i in s.serialize().iter() {
    //     println!("{}: {:?}", &i.0, Vec::from((&i.1).clone()));
    // }
}


