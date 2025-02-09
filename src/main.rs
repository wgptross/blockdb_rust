//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

fn ex() -> Result<Value, Error> {
    let file = File::open("./config/config.json")?;
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)?;
    let database = json_data
        .get("database")
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error invalid"))?;

    Ok(database.clone())
}

// fn read_json_file(path: &str) -> Result<Value, std::io::Error> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let json_data: Value = serde_json::from_reader(reader)?;
//     let user = json_data
//         .get("database")
//         .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error invalid"))?;
//     //return json data
//     Ok(user.clone())
// }

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

fn main() {
    match ex() {
        // match main ex witch reads orignal database values
        // get these values and then match to the parse_config func
        Ok(database) => match parse_config(&database) {
            Ok(host_str) => {
                //string data now callable throughout program
                println!("calling here {}", host_str)
            }
            Err(err) => {
                println!("error handle {}", err)
            }
        },
        //handle error reading file
        Err(err) => {
            println!("error reading file {}", err)
        }
    }

    // match ex() {
    //     Ok(database) => {
    //         println!("database object found");
    //         parse_config(&database);
    //     }
    //     Err(err) => {
    //         println!("error reading or parsing config {}", err);
    //     }
    // }

    // let ex = ex();

    // println!("{:?}", ex);

    // let json_file_path = "./config/config.json";

    // match read_json_file(json_file_path) {
    //     Ok(user) => match user.get("database") {
    //         Some(database) => {
    //             let location = database
    //                 .get("location")
    //                 .unwrap_or(&Value::Null)
    //                 .as_str()
    //                 .unwrap_or("unknown");

    //             println!("location: {}", location);
    //         }
    //         None => {
    //             println!("not found in table");
    //         }
    //     },
    //     Err(e) => {
    //         println!("error reading json {}", e)
    //     }
    // }
    // // let x = read_json_file("./config/config.json");
    // // println!("{:?}", x);
    // //
}
