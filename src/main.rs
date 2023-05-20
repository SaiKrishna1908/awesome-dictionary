/*
    Smart Dictionary
 */

mod model;

use std::error::Error;

extern  crate reqwest;
use crate::model::*;


#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{    
    let word = String::from("hello");
    match reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word)).await {
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
    Ok(())
}
