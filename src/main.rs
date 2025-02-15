//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error, ErrorKind};
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::io::prelude::*;

fn ex() -> Result<Value, Error> {
    let file = File::open("./config/config.json")?;
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)?;
    let database = json_data
        .get("database")
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error invalid"))?;

    Ok(database.clone())
}

struct BlockDB {
    databasefile: String,
}

fn write_to_db(db: &String, file: &str) {
    let mut local_db = String::from(db);
    local_db.push_str("/");
    local_db.push_str(file);

    let path = Path::new(&local_db);
    let dis = path.display();

    let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
        Err(why) => panic!("could not create {} {}", dis, why),
        Ok(file) => file,
    };

    match file.write("testing\n".as_bytes()) {
        Err(why) => panic!("could not write {}, {}", dis, why),
        Ok(_) => println!("success {}", dis),
    };
}

fn test() {
    let path = Path::new("testing.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("could not create {}, {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("could not write to {}: {}", display, why),
        Ok(_) => println!("success {}", display),
    };
}

impl BlockDB {
    fn open(&self) -> &Self {
        println!("{} calling here", &self.databasefile);
        &self
    }

    fn put(&self, value: String, id: u32) {
        println!(
            "{} calling put other: {} at {}",
            &self.databasefile, value, id
        );
    }

    fn get(&self, id: u32) {
        println!("searching {} for id:{}", self.databasefile, id);
    }
}

//this function takes databse Value and parses specific programed data to a string to be called in other functions
fn parse_config(database: &Value) -> Result<String, Error> {
    if let Some(host) = database.get("location").and_then(Value::as_str) {
        // return string value(s)
        Ok(host.to_string())
    } else {
        //handle error
        Err(Error::new(ErrorKind::NotFound, "not found"))
    }
}

fn get_config() -> Result<String, Error> {
    match ex() {
        Ok(database) => match parse_config(&database) {
            Ok(location_str) => {
                Ok(location_str)
                //println!("{}", location_str)
            }
            Err(..) => {
                Err(Error::new(ErrorKind::NotFound, "not found in config {}"))
                //println!("error handle for parsing config")
            }
        },
        Err(..) => {
            Err(Error::new(ErrorKind::NotFound, "error cover Ex()"))
            //println!("error cover")
        }
    }
}

fn main() {
    test();

    match get_config() {
        Ok(location) => {
            let x = location;

            let database_call = BlockDB { databasefile: x };
            let database_main = BlockDB::open(&database_call);
            println!("{}", database_main.databasefile);
            database_call.put(String::from("this"), 12);
            database_call.get(12);

            write_to_db(&database_main.databasefile, "testing");
        }
        Err(err) => {
            println!("error handle file open {}", err);
        }
    }
}
