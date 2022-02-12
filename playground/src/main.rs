use versebase::table::{Table, TableSchema};
use versebase_derive::TableSchema;


#[derive(TableSchema)]
struct Songs {
    id: i32,
    name: String,
    artist_id: String,
}



fn main() {
    let t = Table::<Songs>::new(
        String::from("test"),
        String::from("/home/a/CLionProjects/versebase_playground/data/a.tbl"),
        None,
    );
    Table::<Songs>::schema_info();
}
