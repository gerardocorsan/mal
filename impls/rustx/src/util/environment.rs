use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::env::var;

fn os_env_hashmap() -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (key, val) in env::vars_os() {
        if let (Ok(k), Ok(v)) = (key.into_string(), val.into_string()) {
            map.insert(k, v);
        }
    }

    return map;
}

lazy_static! {
    static ref ENVIRONMENT: HashMap<String, String> = {
        dotenv().ok();
        let map = os_env_hashmap();

        map
    };
}

pub fn get_value(key: &str) -> String {
    let value = var(key).unwrap_or("".to_string());

    value
}

pub fn initialize() {
    println!("Initializing environment ...");

    dotenv().ok();

    if "false".eq(&get_value("SHOW_ENVIRONMENT")) {
        println!("---------- Environment variables ----------");
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
        println!("-------------------------------------------");
    }

    println!("Environment has been successfully initialized.");
}
