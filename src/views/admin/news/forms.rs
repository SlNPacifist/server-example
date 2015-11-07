use std::io::Result;
use iron::prelude::*;
use forms::*;


#[derive(Debug, Clone)]
pub struct NewsForm {
	pub text: String,
	pub header: String,
}

impl NewsForm {
	pub fn from_request(req: &mut Request) -> Result<NewsForm> {
		let body = try!(get_body(req));
		Ok(NewsForm {
			text: try!(parse_single_field(body.get("text"), "text")).to_string(),
			header: try!(parse_single_field(body.get("header"), "header")).to_string(),
		})
	}
}