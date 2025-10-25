use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Deserialize)]
struct AppConfig {
    watch_path: String,
    root_name: Option<String>,
    mappings: HashMap<String, String>,
}

fn load_config() -> AppConfig {
    let content = fs::read_to_string("config.toml")
        .expect("Could not find config.toml in the working directory");
    toml::from_str(&content).expect("TOML is invalid")
}

fn convert_csv(file_path: &PathBuf, config: &AppConfig){
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .expect("Unable to read CSV file");
    let headers = rdr.headers().unwrap().clone();

    for res in rdr.records() {
        let record = res.unwrap();
        let mut root = json!({});
        for (i, value) in record.iter().enumerate() {
            let header = &headers[i];
            let mapped_key = config.mappings.get(header).unwrap(); // TODO proper error handling
            // Function here to make the actual record and add to root.
            insert_in_root(&mut root, mapped_key, value);
        }
        let root2 = root.clone();
        let json = serde_json::to_string_pretty(&root2).unwrap();
        fs::write(format!("{}.json", "placeholder"), &json).unwrap();
    }
}

fn insert_in_root(root: &mut Value, key: &str, value: &str) {
    let mut root = root;
    let header_parts: Vec<&str> = key.split('.').collect();
    for (i, key) in header_parts.iter().enumerate() {
        //check if its the last, then it needs to add value and not just a new json
        if i == header_parts.len() - 1 {
            if let Some(obj) = root.as_object_mut() {
                obj.insert((*key).to_string(), json!(value));
            }
        } else {
            if !root.get(*key).is_some() {
                if let Some(obj) = root.as_object_mut() {
                    obj.insert((*key).to_string(), json!({}));
                }
            }
            // Make sure to get the root as the new made root to nest it.
            root = root.get_mut(*key).unwrap();
        }
    }
}

fn files_in_input() {
    let x = fs::read_dir("./input").expect("Cannot read files in /input");
    for path in x {
        //println!("{:?}",path.unwrap().path().display());
        let file_path = path.unwrap().path();
        let config = load_config();
        convert_csv(&file_path, &config);
    }
}

#[tokio::main]
async fn main() {
    let config = load_config();
    files_in_input();

    println!("{}", config.watch_path);
}
