#[macro_use]
extern crate versebase_derive;

use std::io::Write;
use std::ops::Deref;
use std::path::Path;


use versebase::table::{Table, TableSchema};
use versebase::index::{TableIndex};
use versebase::datatypes::{Int, Str, DateTime, DataType};
use versebase::datatypes;



#[derive(TableSchema)]
struct Songs {
    id: Int,
    name: Str,
    posted_at: DateTime,
}


fn main() {
    let mut table = Table::<Songs>::new(
        String::from("test"),
        Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/a.tbl")),
        Some(TableIndex::new(
            Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/a.idx"))
        ).unwrap()),
    ).unwrap();

    // Table::<Songs>::schema_info();
    //
    let s1 = Songs {
        id: Int::new(1),
        name: Str::new("Season in Abyss".into()),
        posted_at: DateTime::new(chrono::offset::Utc::now().naive_utc())
    };
    let s2 = Songs {
        id: Int::new(2),
        name: Str::new("Easy Way Out".into()),
        posted_at: DateTime::new(chrono::offset::Utc::now().naive_utc())
    };
    let s3 = Songs {
        id: Int::new(3),
        name: Str::new("Saving Us".into()),
        posted_at: DateTime::new(chrono::offset::Utc::now().naive_utc())
    };

    let a = table.create(s1).unwrap_or_else(|e| panic!("Error creating song1, {:?}", e));
    let b = table.create(s2).unwrap();
    let c = table.create(s3).unwrap();

    println!("");

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


