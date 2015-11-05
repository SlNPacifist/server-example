use std::path::Path;
use std::fs::File;
use std::io::Read;
use toml::{Value, Parser, ParserError, Decoder};
use rustc_serialize::Decodable;


#[derive(RustcDecodable, Debug)]
pub struct Config {
	pub http_address: String,
	pub db_connection_string: String,
}

pub fn get_config(path: &Path) -> Config {
	let mut file = File::open(path).expect(
		&format!("Could not open config file {}", path.to_string_lossy()));
	let mut buf = String::new();
	file.read_to_string(&mut buf).expect(
		&format!("Could not read config file {}", path.to_string_lossy()));
	let mut parser = Parser::new(&buf);
	match parser.parse() {
		Some(table) => {
			let value = Value::Table(table);
			let config_opt = Config::decode(&mut Decoder::new(value));
			config_opt.expect(&format!("Could not decode config file {}", path.to_string_lossy()))
		},
		None => {
			let error_messages: Vec<String> = parser.errors.iter()
				.map(|e: &ParserError| format!("\t{}", e))
				.collect();
			panic!(format!(
				"Could not parse config file {}:\n{}",
				path.to_string_lossy(),
				error_messages.join("\n")
			))
		}
	}
}
