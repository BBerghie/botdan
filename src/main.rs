use std::env;
use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config{
    pub discord_token: String,
    pub machine_mac: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the .env file
        dotenv().ok();

        // Leer variables (con unwrap_or para dar errores claros)
        let discord_token = env::var("DISCORD_TOKEN")?;
        let machine_mac = env::var("MACHINE_MAC")?;

        Ok(Self {
            discord_token,
            machine_mac,
        })
    }
}

fn main() {
    match Config::from_env() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
   }
}
