use std::io::Result;
use iron::prelude::*;
use persistent::Read;
use db::Database;
use models::Consumer;
use views::utils::*;
use forms::*;


pub fn add_consumer(req: &mut Request) -> IronResult<Response> {
	let form_opt = AddConsumerForm::from_request(req);
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::consumer::add::add_consumer")
		.get().expect("Could not get connection in admin::consumer::add::add_consumer");
	let loc = match form_opt {
		Ok(form) => {
			Consumer::insert(&connection, form.address);
			"/admin/consumers/?consumer_added"
		}
		Err(_) => "/admin/consumers/?consumer_not_added"
	};
	redirect(loc.to_string())
}

#[derive(Debug, Clone)]
struct AddConsumerForm {
	address: String,
}

impl AddConsumerForm {
	pub fn from_request(req: &mut Request) -> Result<AddConsumerForm> {
		let body = try!(get_body(req));
		Ok(AddConsumerForm {
			address: try!(parse_single_field(body.get("volume"), "address")).to_string(),
		})
	}
}