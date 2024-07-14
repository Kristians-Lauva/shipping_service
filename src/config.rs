use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}


pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:AnnaSieva1@localhost/parcels".to_string())
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config { database_url }
    }
}
