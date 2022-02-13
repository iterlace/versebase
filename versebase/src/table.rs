use std::path::Path;
use std::marker::PhantomData;
use std::collections::HashMap;
use super::index::{TableIndex};
use std::fs;
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::io::ErrorKind::AlreadyExists;

const DELIMITER_SIZE: usize = 8;
const FIELDS_DELIMITER: [u8; DELIMITER_SIZE] = [255, 0, 255, 0, 255, 0, 255, 0];
const ROWS_DELIMITER: [u8; DELIMITER_SIZE] = [0, 127, 0, 255, 0, 127, 0, 255];

pub trait TableSchema {
    fn from_(raw: Vec<(String, Box<[u8]>)>) -> Self;
    fn fields() -> Vec<String>;
    fn print_info();

    fn get_id(&self) -> i32;
    fn serialize_to_vec(&self) -> Vec<(String, Box<[u8]>)>;
    fn serialize_to_map(&self) -> HashMap<String, Box<[u8]>>;
}


struct TableFile<S: TableSchema> {
    pub filepath: Box<Path>,
    pub schema: PhantomData<S>,
    file: File,
}
// File structure looks like
// [row1_field1](FIELDS_DELIMITER)[row1_field2](FIELDS_DELIMITER)[row1_field3](ROWS_DELIMITER)
// [row2_field1](FIELDS_DELIMITER)[row2_field2](FIELDS_DELIMITER)[row2_field3](ROWS_DELIMITER)
// [row3_field1](FIELDS_DELIMITER)[row3_field2](FIELDS_DELIMITER)[row3_field3](ROWS_DELIMITER)

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
        let file = match OpenOptions::new()
            .read(true)
            .append(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path) {
            Ok(f) => f,
            Err(e) => return Result::Err(e),
        };

        Result::Ok(file)
    }

    pub fn seek(&mut self, pos: i64) -> Result<(), std::io::Error> {
        let seek = match pos {
            pos if pos >= 0 => SeekFrom::Start(pos as u64),
            pos => SeekFrom::End(pos + 1)
        };
        return match self.file.seek(seek) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
    }

    pub fn position(&mut self) -> u64 {
        self.file.stream_position().unwrap()
    }

    fn at_beginning(&mut self) -> bool {
        let pos = self.position() as i64;
        pos == 0
    }

    fn at_end(&mut self) -> bool {
        let pos = self.position() as i64;
        pos == self.file.stream_len().unwrap() as i64
    }

    pub fn read_row(&mut self) -> Option<S> {
        if self.at_end() {
            return None;
        }

        // Ensure the pointer is at the ending of the last record
        let pos = self.position() as i64;
        if !self.at_beginning() && !self.at_end() {
            self.seek(pos - ROWS_DELIMITER.len() as i64).unwrap();
            let mut read_delimiter = [0u8; ROWS_DELIMITER.len()];
            let bytes_num = self.file.read(&mut read_delimiter).unwrap();
            // TODO: replace `bytes_num != 0` with an equivalent of `file.size == 0`
            if bytes_num != 0 && (
                bytes_num != ROWS_DELIMITER.len() || read_delimiter != ROWS_DELIMITER
            ) {
                panic!("File pointer is corrupt!");
            }
        }

        let mut buf = Vec::<u8>::with_capacity(16);
        let mut fields_raw = Vec::<Box<[u8]>>::new();

        while !self.at_end() {
            let mut b = [0u8; 1];
            self.file.read_exact(&mut b).unwrap();
            &buf.push(b[0]);

            let possible_delimiter = match buf.len() as i32 - DELIMITER_SIZE as i32 {
                e if e >= 0 => &buf[e as usize..],
                _ => &buf[..]
            };

            let at_row_end = possible_delimiter == ROWS_DELIMITER;
            let at_field_end = possible_delimiter == FIELDS_DELIMITER;

            if at_row_end || at_field_end {
                let field = Box::<[u8]>::from(buf[..&buf.len() - DELIMITER_SIZE].to_vec());
                fields_raw.push(field);
                &buf.clear();
            }
            if at_row_end {
                break;
            }
        }
        let fields_names = S::fields();

        assert_eq!(&fields_raw.len(), &fields_names.len());

        let fields: Vec<(String, Box<[u8]>)> = fields_names
            .iter()
            .zip(fields_raw.iter())
            .map(|e| (e.0.clone(), e.1.clone()))
            .collect()
            ;

        Some(S::from_(fields))
    }

    pub fn write_row(&mut self, row: &S) -> Result<(u64, u64), std::io::Error> {
        match self.seek(-1) {
            Err(e) => return Err(e),
            _ => ()
        };
        let data = row.serialize_to_vec();

        let begin_pos = self.position();
        for i in 0..data.len() {
            self.file.write_all(&data[i].1).unwrap();
            if i != data.len() - 1 {
                self.file.write_all(&FIELDS_DELIMITER).unwrap();
            }
        }
        self.file.write_all(&ROWS_DELIMITER).unwrap();

        let end_pos = self.position();
        return match self.file.sync_data() {
            Ok(_) => Ok((begin_pos, end_pos)),
            Err(e) => Err(e)
        };
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

    pub fn select(&mut self, id: i32) -> Option<S> {
        match &mut self.index {
            Some(index) => {
                return match index.get(id) {
                    Some(pos) => {
                        self.file.seek(pos as i64);
                        match self.file.read_row() {
                            Some(row) => Some(row),
                            None => None,
                        }
                    },
                    None => None,
                }
            }
            None => {
                self.file.seek(0).unwrap();
                loop {
                    match self.file.read_row() {
                        Some(row) if row.get_id() == id => return Some(row),
                        None => return None,
                        _ => continue
                    }
                }
            }
        };
    }

    pub fn create(&mut self, row: S) -> Result<i32, std::io::Error> {
        match &mut self.index {
            Some(index) => {
                if index.exists(row.get_id()) {
                    panic!("id already exists")
                }
                let written_pos = self.file.write_row(&row).unwrap();
                index.set(row.get_id(), written_pos.0);

                return Ok(row.get_id());
            },
            None => {
                let existing = self.select((&row).get_id());
                match existing {
                    Some(_) => panic!("id already exists"),  // TODO: custom DatabaseError
                    None => {
                        self.file.write_row(&row).unwrap();
                        return Ok((&row).get_id());
                    }
                }
            }
        }
    }

    pub fn update(&mut self, row: S) -> Result<i32, std::io::Error> {
        Ok(0)
    }

    pub fn delete(&mut self, row: S) -> Result<i32, std::io::Error> {
        Ok(0)
    }

    pub fn schema_info() {
        S::print_info();
    }
}
