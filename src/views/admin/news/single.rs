use iron::prelude::*;
use persistent::Read;
use db::Database;
use models::News;
use views::utils::*;
use super::NewsHandler;
use super::forms::NewsForm;


pub fn show(req: &mut Request) -> IronResult<Response> {
	let news = req.extensions.remove::<NewsHandler>()
		.expect("Could not get news id in admin::news::singe::show_edit");
	update_var(req, "news", Box::new(news));
	render_ok(req, "admin/news/single.htmt")
}

pub fn save(req: &mut Request) -> IronResult<Response> {
	let form_opt = NewsForm::from_request(req);
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::news::single::save")
		.get().expect("Could not get connection in admin::news::single::save");
	let news = req.extensions.remove::<NewsHandler>()
		.expect("Could not get news id in admin::news::singe::show_edit");
	let is_saved = match form_opt {
		Ok(form) => {
			match News::update(&connection, news.id, form.text, form.header) {
				Err(err) => {
					println!("Could not update news: {}", err);
					false
				}
				Ok(_) => true,
			}
		}
		Err(err) => {
			println!("Could not parse news save form: {}", err);
			false
		}
	};
	redirect(match is_saved {
		true => format!("/admin/news/{}/?news_saved", news.id),
		false => format!("/admin/news/{}/?news_not_saved", news.id),
	})
}