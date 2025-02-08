//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

fn read_json_file(path: &str) -> Result<Value, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)?;
    let user = json_data
        .get("database")
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error invalid"))?;
    //return json data
    Ok(user.clone())
}

fn main() {
    let json_file_path = "./config/config.json";

    match read_json_file(json_file_path) {
        Ok(user) => {
            match user.get("database") {
                Some(database) => {
                    let user = database.get("location").unwrap_or(&Value::Null)as_str().unwrap_or("unknown");


                    println!("location: {}", user);

                },
                None => {
                    println!("not found in table");
                }
            }
        },
        Err(e) => {
            println!("error reading json")
        }
    }


    // let x = read_json_file("./config/config.json");
    // println!("{:?}", x);
    //
    println!("{}", user)
}
