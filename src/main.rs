extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::{Path};

fn main() {
    println!("test");

    let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
    dotenv::from_path(my_path.as_path());

    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }
}
