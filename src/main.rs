use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::runtime::Runtime;
use base64::decode;

#[derive(Serialize, Deserialize, Debug)]
struct DataPrototype {
    pm2_5: u32,
    datetime: String,
    location: String,
    pm10: u32,
    pm1: u32,
}

fn validate_json(json_string: &str) -> Result<DataPrototype, serde_json::Error> {
    // Deserialize the JSON string into Data
    // If it is not valid, this will return an error
    serde_json::from_str(json_string)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a base64-encoded JSON string as the first argument.");
        return Ok(());
    }

    let base64_string = &args[1];

    // decode the base64 string
    let decoded_result = decode(base64_string);

    match decoded_result {
        Ok(json_bytes) => {
            let json_string = String::from_utf8(json_bytes).expect("Failed to convert to UTF8 String");

            let rt = Runtime::new().unwrap();

            // Validate the JSON
            match validate_json(&json_string) {
                Ok(data) => {
                    rt.block_on(async {
                        let client = reqwest::Client::new();

                        // Make a POST request
                        let res = client
                            .post("https://projects.redcloversoftware.ca/nswildfire-airqualitystation/save")
                            .json(&data)
                            .send()
                            .await?;

                        println!("Server response status: {}", res.status());

                        Ok(())
                    })
                }
                Err(e) => {
                    println!("Failed to validate JSON: {}", e);
                    return Ok(());
                }
            }
        }
        Err(_) => {
            println!("Failed to decode base64 string");
            return Ok(());
        }
    }
}