use std::io::{self, BufRead, Write};
use regex::Regex;
use versebase::table::TableSchema;
use super::db::{Database};
use super::schemas::*;
use versebase::datatypes::{Int, Str, DateTime, DataType, DType};
use versebase::datatypes;


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
                    "exit" => break,
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
                Ok(artist) => println!("{}", artist),
                Err(e) => println!("Error: {}", e.message)
            },
            "songs" => match &self.db.songs.get(id) {
                Ok(song) => println!("{}", song),
                Err(e) => println!("Error: {}", e.message)
            },
            table_name => {
                println!("Table \"{}\" is not supported", table_name);
                return;
            }
        }
    }

    fn update(&mut self, command: Command) {
        if command.arguments.len() < 3 {
            println!("Usage: insert [artists, songs] [field1>; <field2>; ...]");
            return;
        }
        let table = (&command.arguments[0]).to_string();

        match table.as_str() {
            "artists" => {
                if command.arguments.len() != 3 {
                    println!("artists table consists of exactly 2 fields. Check your input and try again.");
                    return;
                }
                let id = match command.arguments[1].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid id!", &command.arguments[1]);
                        return;
                    }
                };
                let name = command.arguments[2].clone();

                if let Err(e) = self.db.artists.delete(id.clone()) {
                    println!("Error: {}", e.message);
                    return;
                }

                let artist = Artists {
                    id: Int::new(id),
                    name: Str::new(name),
                };
                match self.db.artists.create(artist) {
                    Ok(_id) => println!("Updated artist with id = {}", _id),
                    Err(e) => println!("Error: {}", e.message)
                }
            },
            "songs" => {
                if command.arguments.len() != 4 {
                    println!("songs table consists of exactly 3 fields. Check your input and try again.");
                    return;
                }
                let id = match command.arguments[1].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid id!", &command.arguments[1]);
                        return;
                    }
                };
                let name = command.arguments[2].clone();
                let artist_id = match command.arguments[3].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid artist foreign key!", &command.arguments[1]);
                        return;
                    }
                };

                // Check if given artist exists
                match self.db.artists.get(artist_id.clone()) {
                    Ok(_) => {},
                    Err(e) => {println!("Error: {}", e.message); return}
                }


                if let Err(e) = self.db.songs.delete(id.clone()) {
                    println!("Error: {}", e.message);
                    return;
                }


                let song = Songs {
                    id: Int::new(id),
                    name: Str::new(name),
                    artist_id: Int::new(artist_id)
                };
                match self.db.songs.create(song) {
                    Ok(_id) => println!("Updated song with id = {}", _id),
                    Err(e) => println!("Error: {}", e.message)
                }
            },
            table_name => {
                println!("Table \"{}\" is not supported", table_name);
                return;
            }
        }
    }

    fn insert(&mut self, command: Command) {
        if command.arguments.len() < 3 {
            println!("Usage: insert [artists, songs] [field1>; <field2>; ...]");
            return;
        }
        let table = (&command.arguments[0]).to_string();

        match table.as_str() {
            "artists" => {
                if command.arguments.len() != 3 {
                    println!("artists table consists of exactly 2 fields. Check your input and try again.");
                    return;
                }
                let id = match command.arguments[1].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid id!", &command.arguments[1]);
                        return;
                    }
                };
                let name = command.arguments[2].clone();

                let artist = Artists {
                    id: Int::new(id),
                    name: Str::new(name),
                };
                match self.db.artists.create(artist) {
                    Ok(_id) => println!("Created artist with id = {}", _id),
                    Err(e) => println!("Error: {}", e.message)
                }
            },
            "songs" => {
                if command.arguments.len() != 4 {
                    println!("songs table consists of exactly 3 fields. Check your input and try again.");
                    return;
                }
                let id = match command.arguments[1].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid id!", &command.arguments[1]);
                        return;
                    }
                };
                let name = command.arguments[2].clone();
                let artist_id = match command.arguments[3].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} is not a valid foreign key!", &command.arguments[1]);
                        return;
                    }
                };

                // Check if given artist exists
                match self.db.artists.get(artist_id.clone()) {
                    Ok(_) => {},
                    Err(e) => {println!("Error: {}", e.message); return}
                }

                let song = Songs {
                    id: Int::new(id),
                    name: Str::new(name),
                    artist_id: Int::new(artist_id)
                };
                match self.db.songs.create(song) {
                    Ok(_id) => println!("Created song with id = {}", _id),
                    Err(e) => println!("Error: {}", e.message)
                }
            },
            table_name => {
                println!("Table \"{}\" is not supported", table_name);
                return;
            }
        }
    }

    fn delete(&mut self, command: Command) {
        if command.arguments.len() != 2 {
            println!("Usage: delete [artists, songs] <id>");
            return;
        }
        println!("delete {:?}", command);

        let id = match command.arguments[1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("{} is not a valid id!", &command.arguments[1]);
                return;
            }
        };

        // let record: Option<dyn TableSchema>;
        match (&command.arguments[0]).as_str() {
            "artists" => match &self.db.artists.delete(id) {
                Ok(_) => println!("Successfully deleted!"),
                Err(e) => println!("Error: {}", e.message)
            },
            "songs" => match &self.db.songs.delete(id) {
                Ok(_) => println!("Successfully deleted!"),
                Err(e) => println!("Error: {}", e.message)
            },
            table_name => {
                println!("Table \"{}\" is not supported", table_name);
                return;
            }
        }
    }

    fn help(&self) {
        println!("List of available commands:\n\
                    \tget [artists, songs] <id>\n\
                    \tupdate [artists, songs] <id> [field1>; <field2>; ...]\n\
                    \tinsert [artists, songs] [field1>; <field2>; ...]\n\
                    \tdelete [artists, songs] <id>\n\
                    \thelp\n\
                    \texit\
        ");
    }
}
