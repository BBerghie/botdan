use std::env;
use dotenvy::dotenv;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Config{
    pub discord_token: String,
    pub machine_mac: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the .env file
        dotenv().ok();

        //Read it
        let discord_token = env::var("DISCORD_TOKEN")?;
        let machine_mac = env::var("MACHINE_MAC")?;

        Ok(Self {
            discord_token,
            machine_mac,
        })
    }
}

// Generating project level constants
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::from_env().expect("One or more env's variable are not defined. Check the '.env.examaple' file.")
});
