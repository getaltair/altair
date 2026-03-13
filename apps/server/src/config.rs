use std::env;

#[derive(Clone, Debug)]
pub struct Config {
	pub database_url: String,
	pub host: String,
	pub port: u16,
	pub log_level: String,
}

impl Config {
	pub fn from_env() -> Self {
		let database_url =
			env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
		let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let port = env::var("PORT")
			.unwrap_or_else(|_| "3000".to_string())
			.parse()
			.expect("PORT must be a valid u16 number");
		let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

		Config {
			database_url,
			host,
			port,
			log_level,
		}
	}
}
