/*
   Smart Dictionary
*/

mod model;

use clap::Parser;

use crate::model::*;
use std::{
    error::Error,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t=String::from(""))]
    meaning: String,
    #[arg(short, long, default_value_t=String::from(""))]
    synonym: String,
    #[arg(short, long, default_value_t=String::from(""))]
    antonym: String,
}

fn get_synonyms(word: &DictionaryWord) -> Vec<String> {
    let mut synonms: Vec<String> = Vec::new();
    for i in 0..word.words.len() {
        let curr_meanings = &word.words[i].meanings;
        let mut meanings = curr_meanings
            .into_iter()
            .map(|f| String::from(&f.synonyms.join("\n")))
            .collect::<Vec<_>>();

        synonms.append(&mut meanings);
    }

    synonms
}

async fn fetch_word_details(word: &String) -> DictionaryWord {
    match reqwest::get(format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word
    ))
    .await
    {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.json::<Vec<Word>>().await {
                    Ok(res) => {
                        return DictionaryWord { words: res };
                    }
                    Err(_) => println!("Failed to parse response to json"),
                }
            }
        }
        Err(_) => println!("Could not make the request"),
    }

    return DictionaryWord { words: Vec::new() };
}

async fn get_defintions(word: &String) -> Vec<String> {
    let mut meanings: Vec<String> = Vec::new();
    match reqwest::get(format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word
    ))
    .await
    {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.json::<Vec<Word>>().await {
                    Ok(res) => {
                        for i in 0..res.len() {
                            meanings.push(res[i].to_string());
                        }
                    }
                    Err(_) => println!("Failed to parse response to json"),
                }
            }
        }
        Err(_) => println!("Could not make the request"),
    }

    meanings
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.meaning.len() != 0 {
        /*
           read input from user using cmd

           let mut lines = io::stdin().lock().lines();
           while let Some(line) = lines.next() {
               let last_input = line.unwrap();

               if last_input == "exit" {
                   break;
               } else {
                   let meanings = get_defintions(&last_input).await;
                   println!("{}", meanings.join("\n"));
               }
           }
        */

        let meanings = get_defintions(&args.meaning).await;
        println!("{}", meanings.join("\n"));
    }

    if !args.synonym.is_empty() {
        let dict_word = fetch_word_details(&args.synonym).await;
        let synonyms = get_synonyms(&dict_word);
        println!("{}", synonyms.join(","));
    }

    Ok(())
}
