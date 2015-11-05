use iron::prelude::*;
use persistent::Read;
use chrono;
use db::Database;
use models::News;
use views::utils::*;
use super::forms::NewsForm;

pub fn process_add_news(req: &mut Request) -> IronResult<Response> {
	let form_opt = NewsForm::from_request(req);
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