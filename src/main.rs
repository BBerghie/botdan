use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    match env::var("MACHINE_MAC") {
        Ok(machine_mac) => println!("the machine's mac is {}", machine_mac),
        Err(e) => println!("Could't read machine_mac ({})", e),
    };
}
