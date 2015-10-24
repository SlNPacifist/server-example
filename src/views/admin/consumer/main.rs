use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use persistent::Read;
use urlencoded::{QueryMap, UrlEncodedBody};
use dtl::{Context, HashMapContext};
use chrono;
use chrono::NaiveDate;
use db::Database;
use models::{VolumePayment, Consumer};
use dtl_impls::VolumePaymentList;
use super::ConsumerHandler;
use views::utils::*;
use forms::*;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let mut ctx = HashMapContext::new();
	{
		let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
		let consumer = req.extensions.get::<ConsumerHandler>().unwrap();
		let payments = VolumePayment::for_consumer(&connection, consumer.id);
		let mut volume_sum = 0.0;
		let mut money_sum = 0.0;
		for p in payments.iter() {
			volume_sum += p.volume;
			money_sum += p.sum;
		}
		ctx.set("consumer", Box::new(consumer.clone()));
		ctx.set("payments", Box::new(VolumePaymentList::new(payments)));
		ctx.set("total_volume_sum", Box::new(volume_sum));
		ctx.set("total_money_sum", Box::new(money_sum));
		ctx.set("today", Box::new(chrono::Local::today()));
	}
	
	render_ok(req, &ctx, "admin/consumer.htmt")
}

pub fn add_payment(req: &mut Request) -> IronResult<Response> {
	let form_opt = AddPaymentForm::new(&req.get::<UrlEncodedBody>().unwrap());
	let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
    let consumer = req.extensions.get::<ConsumerHandler>().unwrap();
	let loc = match form_opt {
		Ok(form) => {
			VolumePayment::insert(&connection, form.volume, consumer.id, form.payment_date, form.payment_sum);
			format!("/admin/consumer/{}/?payment_added", consumer.id)
		}
		Err(_) => format!("/admin/consumer/{}/?payment_not_added", consumer.id)
	};
	redirect(loc)
}

pub fn add_consumer(req: &mut Request) -> IronResult<Response> {
	let form_opt = AddConsumerForm::new(&req.get::<UrlEncodedBody>().unwrap());
	let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
	let loc = match form_opt {
		Ok(form) => {
			Consumer::insert(&connection, form.address);
			"/admin/?consumer_added"
		}
		Err(_) => "/admin/?consumer_not_added"
	};
	redirect(loc.to_string())
}

#[derive(Debug, Clone)]
struct AddPaymentForm {
	volume: f32,
	payment_sum: f32,
	payment_date: NaiveDate,
}

impl AddPaymentForm {
	fn get_volume(source: Option<&Vec<String>>) -> Result<f32> {
		let volume = try!(parse_single_f32(source, "volume"));
		if volume <= 1e-6 {
			Err(Error::new(ErrorKind::InvalidInput, format!("volume field in add payment form is too small: {}", volume)))
		} else {
			Ok(volume)
		}
	}
	fn get_payment_sum(source: Option<&Vec<String>>) -> Result<f32> {
		let sum = try!(parse_single_f32(source, "payment_sum"));
		if sum < 0.0 {
			Err(Error::new(ErrorKind::InvalidInput, format!("payment_sum field in add payment form is too small: {}", sum)))
		} else {
			Ok(sum)
		}
	}
	fn get_payment_date(source: Option<&Vec<String>>) -> Result<NaiveDate> {
		let source_string = try!(parse_single_field(source, "payment_date"));
		match NaiveDate::parse_from_str(&source_string, "%Y-%m-%d") {
			Ok(res) => Ok(res),
			Err(err) => Err(Error::new(
				ErrorKind::InvalidInput,
				format!("payment_date field in add payment form could not be parsed: {}", err)
			))
		}
	}
	pub fn new(source: &QueryMap) -> Result<AddPaymentForm> {
		Ok(AddPaymentForm {
			volume: try!(Self::get_volume(source.get("volume"))),
			payment_sum: try!(Self::get_payment_sum(source.get("payment_sum"))),
			payment_date: try!(Self::get_payment_date(source.get("payment_date"))),
		})
	}
}

#[derive(Debug, Clone)]
struct AddConsumerForm {
	address: String,
}

impl AddConsumerForm {
	pub fn new(source: &QueryMap) -> Result<AddConsumerForm> {
		Ok(AddConsumerForm {
			address: try!(parse_single_field(source.get("volume"), "address")).to_string(),
		})
	}
}