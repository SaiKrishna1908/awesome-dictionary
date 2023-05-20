/*
    Smart Dictionary
 */

mod model;

use std::{error::Error, io::{self, BufRead}};
use crate::model::*;



#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{    
    let mut lines = io::stdin().lock().lines();
    

    // let stdin = io::stdin();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input == "exit" {
            break;
        } else {
            match reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", last_input)).await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        match response.json::<Vec<Word>>().await {
                            Ok(res) => {
                                for i in 0..res.len() {
                                    println!("{}",res[i].to_string())
                                }
                            },
                            Err(_) => println!("Failed to parse respnose to json")
                        }                
                    }
                },
                Err(_) => println!("Could not make the request")
            }
        }
    }
    
    
    Ok(())
}
