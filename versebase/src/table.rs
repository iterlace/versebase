//extern crate derive_table_schema;
use std::marker::PhantomData;
use super::index::{TableIndex};
use std::fs;
use std::io;
use std::fs::{File};


pub trait TableSchema {
    fn info();
}

pub struct Table<S: TableSchema> {
    pub name: String,
    pub index: Option<TableIndex>,
    file: File,
    schema: PhantomData<S>,
}

impl <S: TableSchema> Table<S> {

    pub fn new(
        name: String,
        filepath: String,
        index: Option<TableIndex>,
    ) -> Table<S> {
        let file = Table::<S>::init_file(filepath).expect("Failed to init file.");

        Table {
            name,
            index,
            file,
            schema: PhantomData
        }
    }

    pub fn schema_info() {
        S::info();
    }

    fn init_file(path: String) -> Result<File, io::Error> {
        let file: File;

        if fs::metadata(&path).is_ok() {
            file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => return Result::Err(e),
            };
        } else {
            file = match File::create(&path) {
                Ok(f) => f,
                Err(e) => return Result::Err(e),
            };
        }

        Result::Ok(file)
    }
}
