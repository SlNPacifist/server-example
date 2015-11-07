use std::io::{Result, Error, ErrorKind};
use std::str::FromStr;
use iron::prelude::*;
use urlencoded::{QueryMap, UrlEncodedBody};


pub fn parse_single_field<'a>(source: Option<&'a Vec<String>>, field_name: &str) -> Result<&'a str> {
	match source {
		Some(strings) => {
			match strings.len() {
				1 => Ok(&strings[0]),
				0 => Err(Error::new(ErrorKind::InvalidInput, format!("{} field is empty", field_name))),
				_ => Err(Error::new(ErrorKind::InvalidInput, format!("Not single value for {} field", field_name)))
			}
		},
		None => Err(Error::new(ErrorKind::InvalidInput, format!("No {} field", field_name)))
	}
}

pub fn parse_single_f32(source: Option<&Vec<String>>, field_name: &str) -> Result<f32> {
	match f32::from_str(try!(parse_single_field(source, field_name))) {
		Ok(val) => Ok(val),
		Err(reason) => Err(Error::new(ErrorKind::InvalidInput, format!("Could not parse {} field: {}", field_name, reason))),
	}
}

pub fn parse_single_i32(source: Option<&Vec<String>>, field_name: &str) -> Result<i32> {
	match i32::from_str(try!(parse_single_field(source, field_name))) {
		Ok(val) => Ok(val),
		Err(reason) => Err(Error::new(ErrorKind::InvalidInput, format!("Could not parse {} field: {}", field_name, reason))),
	}
}

pub fn get_body(req: &mut Request) -> Result<QueryMap> {
	req.get::<UrlEncodedBody>().map_err(|e| {
		Error::new(ErrorKind::InvalidInput, format!("Could not get request body: {}", e))
	})
}