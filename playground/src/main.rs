#[macro_use]
extern crate versebase_derive;

use std::ops::Deref;
use std::path::Path;


use versebase::table::{Table, TableSchema};
use versebase::datatypes::{Int, Str, DateTime, DataType};
use versebase::datatypes;



#[derive(TableSchema)]
struct Songs {
    id: Int,
    name: Str,
    posted_at: DateTime,
}



fn main() {
    let t = Table::<Songs>::new(
        String::from("test"),
        Box::from(Path::new("/home/a/CLionProjects/versebase_playground/data/a.tbl")),
        None,
    );
    Table::<Songs>::schema_info();

    let s = Songs {
        id: Int::new(1),
        name: Str::new("Season in Abyss".into()),
        posted_at: DateTime::new(chrono::offset::Utc::now().naive_utc())
    };

    for i in s.serialize().iter() {
        println!("{}: {:?}", &i.0, Vec::from((&i.1).clone()));
    }
}


