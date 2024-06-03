extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub fn setup() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
