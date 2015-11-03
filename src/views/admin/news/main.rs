use iron::prelude::*;
use persistent::Read;
use db::Database;
use models::News;
use dtl_impls::NewsList;
use views::utils::*;


pub fn all_news(req: &mut Request) -> IronResult<Response> {
	{
		let connection = req.get::<Read<Database>>()
			.expect("Could not get connection pool in admin::news::all_news")
			.get().expect("Could not get connection in admin::news::all_news");
		let news = News::ordered_by_date(&connection, 20);
		update_var(req, "news", Box::new(NewsList::new(news))); 
	}
	render_ok(req, "admin/news/list.htmt")
}