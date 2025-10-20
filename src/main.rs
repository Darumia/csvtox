use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct AppConfig{
    watch_path:String,
}

fn load_config() -> AppConfig {
    let content = fs::read_to_string("config.toml").expect("Could not find config.toml in the working directory");
    toml::from_str(&content).expect("TOML is invalid")
}

#[tokio::main]
async fn main() {
    let config = load_config();

    println!("{}", config.watch_path);
}
