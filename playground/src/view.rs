use std::io::{self, BufRead, Write};
use regex::Regex;
use versebase::table::TableSchema;
use super::db::{Database};


pub struct Playground {
    db: Database,
}


#[derive(Debug)]
struct Command {
    name: String,
    arguments: Vec<String>,
}


impl Playground {
    pub fn new() -> Self {
        Self {
            db: Database::new()
        }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            match self.parse_db_operation(input.as_str()) {
                Some(command) => match command.name.as_str() {
                    "get" => self.get(command),
                    "update" => self.update(command),
                    "insert" => self.insert(command),
                    "delete" => self.delete(command),
                    "help" => self.help(),
                    _ => self.help(),
                },
                None => {
                    println!("Unrecognized command.");
                    self.help();
                }
            }
        }
    }

    fn parse_db_operation(&self, input: &str) -> Option<Command> {
        let command_name_re = Regex::new(r#"^\s*?(\w+)\s*?.*?$"#).unwrap();
        let argument_re = Regex::new(r#"\s*?([\w_]+)\s*?"#).unwrap();

        let command_name_cap = command_name_re
            .captures(&input)?
            .get(1)?;

        let command_name = command_name_cap.as_str().to_string();

        let arguments: Vec<String> = argument_re
            .captures_iter(&input[command_name_cap.end()..])
            .map(|m| m.get(1).unwrap().as_str().to_string())
            .collect()
            ;

        Some(Command { name: command_name, arguments })
    }

    fn get(&mut self, command: Command) {
        if command.arguments.len() != 2 {
            println!("Usage: get [artists, songs] <id>");
            return;
        }

        let id = match command.arguments[1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("{} is not a valid id!", &command.arguments[1]);
                return;
            }
        };

        // let record: Option<dyn TableSchema>;
        match (&command.arguments[0]).as_str() {
            "artists" => match &self.db.artists.get(id) {
                Some(artist) => println!("{}", artist),
                None => {}
            },
            "songs" => match &self.db.songs.get(id) {
                Some(song) => println!("{}", song),
                None => {}
            },
            table_name => {
                println!("Table \"{}\" is not supported", table_name);
                return;
            }
        }
    }

    fn update(&self, command: Command) {
        println!("update {:?}", command);
    }

    fn insert(&self, command: Command) {
        println!("insert {:?}", command);
    }

    fn delete(&self, command: Command) {
        if command.arguments.len() != 2 {
            println!("Usage: delete [artists, songs] <id>");
            return;
        }
        println!("delete {:?}", command);
    }

    fn help(&self) {
        println!("List of available commands:\n\
                    \tget [artists, songs] <id>\n\
                    \tupdate [artists, songs] <id>\n\
                    \tinsert [artists, songs] <id>\n\
                    \tdelete [artists, songs] <id>\n\
                    \thelp\
        ");
    }
}
