use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct DictionaryWord {
    pub words: Vec<Word>
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
    pub license: License2,
    pub source_urls: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phonetic {
    pub audio: String,
    pub source_url: Option<String>,
    pub license: Option<License>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meaning {
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub definition: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub example: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License2 {
    pub name: String,
    pub url: String,
}

impl ToString for Word {
    fn to_string(&self) -> String {
        let mut meanings: Vec<String> = Vec::new();
        let mut index: i32 = 1;

        for i in 0..self.meanings.len() {
            for j in 0..self.meanings[i].definitions.len() {
                meanings.push(format!("{}. {}",index, self.meanings[i].definitions[j].definition.to_owned()) );
                index+=1;
            }            
        }

        let meaning_string = meanings.join("\n");

        return  meaning_string;
    }
}

impl ToString for Meaning {
    fn to_string(&self) -> String {
        let mut definitions: Vec<String> = Vec::new();

        for i in 0..self.definitions.len() {
            definitions.push(self.definitions[i].to_string()) 
        }

        let def_string = definitions.join("\n");
        let formatted_string = format!("parts of speech: {}\nSynonyms: {}\nAntonyms: {}\nDefinition: {}", self.part_of_speech, self.synonyms.join(","), self.antonyms.join(","), def_string);

        return formatted_string;
    }
}

impl ToString for Definition {
    fn to_string(&self) -> String {
        let formatted_string = format!(
            "definition: {}\nsynonyms: {}\nantonyms: {}\nexample: {}",
            self.definition,
            self.synonyms.join(","),
            self.antonyms.join(","),
            String::from("example")
        );
        return formatted_string;
    }
}
