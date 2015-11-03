use std::io::Result;
use iron::prelude::*;
use persistent::Read;
use urlencoded::{QueryMap, UrlEncodedBody};
use chrono;
use db::Database;
use models::News;
use views::utils::*;
use forms::*;


pub fn procerss_add_news(req: &mut Request) -> IronResult<Response> {
	let form_opt = AddNewsForm::new(
		&req.get::<UrlEncodedBody>()
			.expect("Could not get request body in admin::news::add_news")
	);
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::news::add_news")
		.get().expect("Could not get connection in admin::news::add_news");
	let loc = match form_opt {
		Ok(form) => {
			let today = chrono::Local::today().naive_local();
			News::insert(
				&connection, form.text, form.header, today);
			"/admin/news/?news_added"
		}
		Err(_) => "/admin/news/?news_not_added"
	};
	redirect(loc.to_string())
}

pub fn add_news(req: &mut Request) -> IronResult<Response> {
	render_ok(req, "admin/news/add.htmt")
}

#[derive(Debug, Clone)]
struct AddNewsForm {
	text: String,
	header: String,
}

impl AddNewsForm {
	pub fn new(source: &QueryMap) -> Result<AddNewsForm> {
		Ok(AddNewsForm {
			text: try!(parse_single_field(source.get("text"), "text")).to_string(),
			header: try!(parse_single_field(source.get("header"), "header")).to_string(),
		})
	}
}