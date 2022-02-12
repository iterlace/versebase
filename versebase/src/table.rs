use std::path::Path;
use std::marker::PhantomData;
use super::index::{TableIndex};
use std::fs;
use std::io;
use std::fs::{File};
use std::io::{Seek, SeekFrom};


pub trait TableSchema {
    fn info();

    fn serialize(&self) -> std::vec::Vec<(String, Box<[u8]>)>;
}


struct TableFile<S: TableSchema> {
    pub filepath: Box<Path>,
    pub schema: PhantomData<S>,
    file: File,
}

impl<S: TableSchema> TableFile<S> {
    pub fn new(filepath: Box<Path>) -> Result<Self, io::Error> {
        let file = match Self::init_file(&filepath) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        Ok(TableFile {
            filepath,
            schema: PhantomData,
            file,
        })
    }

    fn init_file(path: &Box<Path>) -> Result<File, io::Error> {
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



    fn seek(&mut self, pos: usize) -> Result<(), std::io::Error> {
        return match self.file.seek(SeekFrom::Start(pos as u64)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}


pub struct Table<S: TableSchema> {
    pub name: String,
    pub index: Option<TableIndex>,
    file: TableFile<S>,
    schema: PhantomData<S>,
}

impl<S: TableSchema> Table<S> {
    pub fn new(
        name: String,
        filepath: Box<Path>,
        index: Option<TableIndex>,
    ) -> Result<Table<S>, io::Error> {
        let file = match TableFile::<S>::new(filepath.clone()) {
            Ok(f) => f,
            Err(e) => return Err(e)
        };

        Ok(Table {
            name,
            index,
            file,
            schema: PhantomData,
        })
    }

    pub fn schema_info() {
        S::info();
    }
}
