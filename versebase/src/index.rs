use std::path::Path;
use std::marker::PhantomData;
use std::collections::{HashMap, BTreeMap};
use std::{fs, u64};
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::io::ErrorKind::AlreadyExists;
use std::ops::Deref;
use rand::distributions::uniform::SampleBorrow;


pub struct TableIndex {
    pub filepath: Box<Path>,
    // tree of (id, file_pos)
    tree: BTreeMap<i32, u64>,
    file: File,
}

impl TableIndex {
    pub fn new(filepath: Box<Path>) -> Result<Self, io::Error> {
        let file = match Self::init_file(&filepath) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut instance = TableIndex {
            filepath,
            tree: BTreeMap::new(),
            file,
        };
        instance.load();

        Ok(instance)
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

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        self.file.seek(SeekFrom::Start(0))?;
        while !(self.file.stream_position().unwrap() as i64 == self.file.stream_len().unwrap() as i64) {
            let mut id_buf = [0u8; 4];
            let mut pos_buf = [0u8; 8];

            self.file.read_exact(&mut id_buf).unwrap();
            self.file.read_exact(&mut pos_buf).unwrap();

            let id = i32::from_ne_bytes(id_buf.try_into().unwrap());
            let pos = u64::from_ne_bytes(pos_buf.try_into().unwrap());

            self.tree.insert(id, pos);
        }

        Ok(())
    }

    fn dump(&mut self) -> Result<(), std::io::Error> {
        self.file.set_len(0)?;
        for (id, pos) in &self.tree {
            self.file.write(&id.to_ne_bytes())?;
            self.file.write(&pos.to_ne_bytes())?;
        }
        self.file.sync_data().unwrap();

        Ok(())
    }

    pub fn exists(&self, id: i32) -> bool {
        self.tree.contains_key(&id)
    }

    pub fn get(&self, id: i32) -> Option<u64> {
        match self.tree.get(&id) {
            Some(e) => Some(*e),
            None => None
        }
    }

    pub fn set(&mut self, id: i32, pos: u64) {
        self.tree.insert(id, pos);
        self.dump().unwrap();
    }

    pub fn delete(&mut self, id: i32) -> Option<u64> {
        let result = self.tree.remove(&id);
        self.dump().unwrap();
        result
    }
}

impl Drop for TableIndex {
    fn drop(&mut self) {
        self.dump();
    }
}

