use std::path::Path;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::io::ErrorKind::AlreadyExists;
use std::any::Any;

use super::error::{self, Error, ErrorKind};
use super::index::{TableIndex};
use super::datatypes::{DataType, DType};

const DELIMITER_SIZE: usize = 8;
const FIELDS_DELIMITER: [u8; DELIMITER_SIZE] = [255, 0, 255, 0, 255, 0, 255, 0];
const ROWS_DELIMITER: [u8; DELIMITER_SIZE] = [0, 127, 0, 255, 0, 127, 0, 255];

pub trait TableSchema: fmt::Display {
    fn from_(raw: Vec<(String, Box<[u8]>)>) -> Self;
    fn fields() -> Vec<String>;
    fn print_info();

    fn get(&self, field: String) -> Option<DType>;
    fn get_id(&self) -> i32;
    fn to_map(&self) -> HashMap<String, DType>;
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
    pub fn new(filepath: Box<Path>) -> Result<Self, Error> {
        let file = match Self::init_file(&filepath) {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
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

    pub fn seek(&mut self, pos: i64) -> Result<(), Error> {
        let seek = match pos {
            pos if pos >= 0 => SeekFrom::Start(pos as u64),
            pos => SeekFrom::End(pos + 1)
        };
        return match self.file.seek(seek) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        };
    }

    pub fn position(&mut self) -> u64 {
        self.file.stream_position().unwrap()
    }

    fn at_beginning(&mut self) -> bool {
        let pos = self.position() as i64;
        pos == 0
    }

    fn at_end(&mut self) -> Result<bool, Error> {
        let pos = self.position() as i64;
        Ok(pos == self.file.stream_len()? as i64)
    }

    pub fn read_row(&mut self) -> Result<Option<(S, u64, u64)>, Error> {
        if self.at_end()? {
            return Ok(None);
        }

        // Ensure the pointer is at the ending of the last record
        let pos_begin = self.position() as i64;
        if !self.at_beginning() && !self.at_end()? {
            self.seek(pos_begin - DELIMITER_SIZE as i64)?;
            let mut read_delimiter = [0u8; DELIMITER_SIZE];
            let bytes_num = self.file.read(&mut read_delimiter)?;
            // TODO: replace `bytes_num != 0` with an equivalent of `file.size == 0`
            if bytes_num != 0 && (
                bytes_num != DELIMITER_SIZE || read_delimiter != ROWS_DELIMITER
            ) {
                return Err(Error {
                    kind: ErrorKind::FilePointerCorrupt,
                    message: "file pointer is corrupt".to_string()
                });
            }
        }

        let mut buf = Vec::<u8>::with_capacity(16);
        let mut fields_raw = Vec::<Box<[u8]>>::new();

        while !self.at_end()? {
            let mut b = [0u8; 1];
            self.file.read_exact(&mut b)?;
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
        let pos_end = self.position();

        let fields_names = S::fields();

        assert_eq!(&fields_raw.len(), &fields_names.len());

        let fields: Vec<(String, Box<[u8]>)> = fields_names
            .iter()
            .zip(fields_raw.iter())
            .map(|e| (e.0.clone(), e.1.clone()))
            .collect()
            ;

        Ok(Some((S::from_(fields), pos_begin as u64, pos_end)))
    }

    pub fn write_row(&mut self, row: &S) -> Result<(u64, u64), Error> {
        match self.seek(-1) {
            Err(e) => return Err(e.into()),
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
            Err(e) => Err(e.into())
        };
    }

    pub fn erase(&mut self, begin: u64, end: u64) -> Result<(), Error> {
        assert!(begin < end);

        // Save data after the "end" pointer
        self.file.seek(SeekFrom::Start(end));
        let mut buf = Vec::<u8>::with_capacity((self.file.stream_len()? - end) as usize);
        self.file.read_to_end(&mut buf);

        // Crop file after "end"
        self.file.set_len(begin);
        self.file.seek(SeekFrom::End(0));
        // Write all the saved subsequent data
        self.file.write_all(&buf);

        self.file.flush()?;

        Ok(())
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
    ) -> Result<Table<S>, Error> {
        let file = match TableFile::<S>::new(filepath.clone()) {
            Ok(f) => f,
            Err(e) => return Err(e)
        };

        let mut table = Table {
            name,
            index,
            file,
            schema: PhantomData,
        };
        table.refresh_indexes();

        Ok(table)
    }

    pub fn get(&mut self, id: i32) -> Result<S, Error> {
        match &mut self.index {
            Some(index) => {
                return match index.get(id) {
                    Some(pos) => {
                        self.file.seek(pos as i64);
                        match self.file.read_row() {
                            Ok(Some((row, _, _))) => Ok(row),
                            Ok(None) => Err(Error {
                                kind: ErrorKind::NotFound,
                                message: "record with a given id doesn't exist".to_string()
                            }),
                            Err(e) => Err(e)
                        }
                    }
                    None => Err(Error {
                        kind: ErrorKind::NotFound,
                        message: "record with a given id doesn't exist".to_string()
                    }),
                };
            }
            None => {
                self.file.seek(0)?;
                loop {
                    match self.file.read_row()? {
                        Some((row, _, _)) if row.get_id() == id => return Ok(row),
                        None => return Err(Error {
                            kind: ErrorKind::NotFound,
                            message: "record with a given id doesn't exist".to_string()
                        }),
                        _ => continue
                    }
                }
            }
        };
    }

    pub fn select(&mut self, filter: HashMap<String, DType>) -> Result<Vec<S>, Error> {
        self.file.seek(0)?;

        let mut result = Vec::<S>::new();
        loop {
            match self.file.read_row()? {
                Some((row, _, _)) => {
                    let mut is_valid = true;
                    for (filter_field, filter_value) in filter.iter() {
                        match &row.get(filter_field.to_string()) {
                            Some(value) if value != filter_value => {
                                is_valid = false;
                                break;
                            }
                            _ => continue
                        }
                    }
                    if is_valid {
                        result.push(row);
                    }
                }
                None => break
            }
        };

        Ok(result)
    }

    pub fn create(&mut self, row: S) -> Result<i32, Error> {
        return match &mut self.index {
            Some(index) => {
                if index.exists(row.get_id()) {
                    return Err(Error {
                        kind: ErrorKind::AlreadyExists,
                        message: "id already exists".to_string()
                    })
                }
                let written_pos = self.file.write_row(&row).unwrap();
                index.set(row.get_id(), written_pos.0);

                Ok(row.get_id())
            }
            None => {
                let existing = self.get((&row).get_id());
                match existing {
                    Ok(_) => Err(Error {
                        kind: ErrorKind::AlreadyExists,
                        message: "id already exists".to_string()
                    }),
                    Err(Error {kind: ErrorKind::AlreadyExists, .. }) => {
                        self.file.write_row(&row).unwrap();
                        Ok((&row).get_id())
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }

    pub fn update(&mut self, row: S) -> Result<(), Error> {
        let pos = match self.find(row.get_id())? {
            Some(e) => e,
            None =>  return Err(Error {kind: ErrorKind::NotFound, message: "record not found".to_string()}),
        };
        return Ok(());
    }

    pub fn delete(&mut self, id: i32) -> Result<(), Error> {
        let (row, begin, end) = match self.find(id)? {
            Some(e) => e,
            None => return Err(Error {kind: ErrorKind::NotFound, message: "record not found".to_string()}),
        };
        self.file.erase(begin, end)?;
        self.refresh_indexes();

        return Ok(());
    }

    /// Returns a tuple of (TableSchema, begin, end), where begin & end are byte-level dimensions
    /// of a given row.
    fn find(&mut self, id: i32) -> Result<Option<(S, u64, u64)>, Error> {
        self.file.seek(0)?;

        loop {
            match self.file.read_row()? {
                Some((row, begin, end)) if row.get_id() == id => {
                    return Ok(Some((row, begin, end)));
                }
                None => return Ok(None),
                _ => continue
            }
        }
    }

    fn refresh_indexes(&mut self) -> Result<(), Error> {
        let mut index = match &mut self.index {
            Some(i) => i,
            None => return Ok(()),
        };

        self.file.seek(0);
        index.clear();

        loop {
            match self.file.read_row()? {
                Some((row, begin, end)) => {
                    index.set(row.get_id(), begin);
                }
                None => break,
            }
        };

        Ok(())
    }

    pub fn schema_info() {
        S::print_info();
    }
}
