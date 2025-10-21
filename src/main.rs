use csv::ReaderBuilder;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct AppConfig{
    watch_path:String,
}

fn load_config() -> AppConfig {
    let content = fs::read_to_string("config.toml").expect("Could not find config.toml in the working directory");
    toml::from_str(&content).expect("TOML is invalid")
}

fn convert_csv(file_path: &PathBuf) {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .expect("Unable to read CSV file");
    let headers = rdr.headers().unwrap().clone();
    println!("{:?}", headers);
}

fn files_in_input(){

    let x = fs::read_dir("./input").expect("Cannot read files in /input");
    for path in x {
        //println!("{:?}",path.unwrap().path().display());
        let file_path = path.unwrap().path();
        convert_csv(&file_path);
    }
}

#[tokio::main]
async fn main() {
    let config = load_config();
    files_in_input();

    println!("{}", config.watch_path);
}
