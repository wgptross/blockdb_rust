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

    Ok(user.clone())
}

fn main() {
    let x = read_json_file("./config/config.json");
    println!("{:?}", x);
}
