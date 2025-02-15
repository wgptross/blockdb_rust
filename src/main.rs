//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::path::Path;
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

fn write_to_path(path: &'static str) {
    let path_input = Path::new(path);

    let display = path_input.display();

    let mut file = match File::open(&path_input) {
        Err(why) => panic!("could not read {} {}", display, why),
        Ok(file) => file,
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
    match get_config() {
        Ok(location) => {
            let x = location;

            let database_call = BlockDB { databasefile: x };
            let database_main = BlockDB::open(&database_call);
            println!("{}", database_main.databasefile);
            database_call.put(String::from("this"), 12);
            database_call.get(12);
        }
        Err(err) => {
            println!("error handle file open {}", err);
        }
    }
}
