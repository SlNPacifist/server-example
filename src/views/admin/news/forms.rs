use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use urlencoded::UrlEncodedBody;
use forms::*;


#[derive(Debug, Clone)]
pub struct NewsForm {
	pub text: String,
	pub header: String,
}

impl NewsForm {
	pub fn from_request(req: &mut Request) -> Result<NewsForm> {
		let body;
		match req.get::<UrlEncodedBody>() {
			Ok(b) => body = b,
			Err(e) => return Err(Error::new(ErrorKind::InvalidInput,
				format!("Could not get request body: {}", e)))
		};
		Ok(NewsForm {
			text: try!(parse_single_field(body.get("text"), "text")).to_string(),
			header: try!(parse_single_field(body.get("header"), "header")).to_string(),
		})
	}
}