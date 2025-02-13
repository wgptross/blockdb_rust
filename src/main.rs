//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Write};

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


impl BlockDB { 
    fn open(&self) -> &Self { 
        println!("{} calling here", &self.databasefile);
        &self
    } 

    fn put(&self, value: String, id: u32) { 
        println!("{} calling put other: {} at {}", &self.databasefile, value, id);
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
            println!("{} call x", x);
            let database_caller = BlockDB {databasefile: x};
            let database_main = BlockDB::open(&database_caller);
            println!("{}", database_main.databasefile);
            database_main.put(String::from("other"), 12);
            database_main.get(12);
        }
        Err(err) => {
            println!("error handle {}", err);
        }
    }
    let cont = true;
    while cont {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);

        println!("{}", buffer);
    }
}
