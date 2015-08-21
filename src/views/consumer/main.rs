use std::path::Path;
use std::io::{Result, Error, ErrorKind};
use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use urlencoded::{QueryMap, UrlEncodedBody};
use dtl::{Context, HashMapContext};
use chrono;
use chrono::NaiveDate;
use db::Database;
use models::VolumePayment;
use dtl_impls::VolumePaymentList;
use super::ConsumerHandler;
use views::TemplateCompilerKey;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let mut ctx = HashMapContext::new();
    let consumer = req.extensions.get::<ConsumerHandler>().unwrap();
	let payments = VolumePayment::for_consumer(&connection, consumer.id);
	let mut volume_sum = 0.0;
	for p in payments.iter() {
		volume_sum += p.volume;
	}
	ctx.set("consumer", Box::new(consumer.clone()));
	ctx.set("payments", Box::new(VolumePaymentList::new(payments)));
	ctx.set("total_volume_sum", Box::new(volume_sum));
	ctx.set("today", Box::new(chrono::Local::today()));
    let response_text = template_compiler.render(Path::new("consumer.htmt"), &ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn add_payment(req: &mut Request) -> IronResult<Response> {
	let form = AddPaymentForm::new(req.get::<UrlEncodedBody>().unwrap());
	println!("{:?}", form);
	Ok(Response::with(status::Ok))
}

#[derive(Debug, Clone)]
struct AddPaymentForm {
	volume: f32,
	payment_sum: f32,
	payment_date: NaiveDate,
}

impl AddPaymentForm {
	fn parse_single_field<'a>(source: Option<&'a Vec<String>>, field_name: &str) -> Result<&'a str> {
		match source {
			Some(strings) => {
				match strings.len() {
					1 => Ok(&strings[0]),
					0 => Err(Error::new(ErrorKind::InvalidInput, format!("{} field in add payment form is empty", field_name))),
					_ => Err(Error::new(ErrorKind::InvalidInput, format!("Not single value for {} field in add payment form", field_name)))
				}
			},
			None => Err(Error::new(ErrorKind::InvalidInput, format!("No {} field in add payment form", field_name)))
		}
	}
	fn parse_single_f32(source: Option<&Vec<String>>, field_name: &str) -> Result<f32> {
		match f32::from_str(try!(Self::parse_single_field(source, field_name))) {
			Ok(val) => Ok(val),
			Err(reason) => Err(Error::new(ErrorKind::InvalidInput, format!("Could not parse {} field: {}", field_name, reason))),
		}
	}
	fn get_volume(source: Option<&Vec<String>>) -> Result<f32> {
		let volume = try!(Self::parse_single_f32(source, "volume"));
		if volume <= 1e-6 {
			Err(Error::new(ErrorKind::InvalidInput, format!("volume field in add payment form is too small: {}", volume)))
		} else {
			Ok(volume)
		}
	}
	fn get_payment_sum(source: Option<&Vec<String>>) -> Result<f32> {
		let sum = try!(Self::parse_single_f32(source, "payment_sum"));
		if sum < 0.0 {
			Err(Error::new(ErrorKind::InvalidInput, format!("payment_sum field in add payment form is too small: {}", sum)))
		} else {
			Ok(sum)
		}
	}
	fn get_payment_date(source: Option<&Vec<String>>) -> Result<NaiveDate> {
		let source_string = try!(Self::parse_single_field(source, "payment_date"));
		match NaiveDate::parse_from_str(&source_string, "%Y-%m-%d") {
			Ok(res) => Ok(res),
			Err(err) => Err(Error::new(
				ErrorKind::InvalidInput,
				format!("payment_date field in add payment form could not be parsed: {}", err)
			))
		}
	}
	pub fn new(source: QueryMap) -> Result<AddPaymentForm> {
		Ok(AddPaymentForm {
			volume: try!(Self::get_volume(source.get("volume"))),
			payment_sum: try!(Self::get_payment_sum(source.get("payment_sum"))),
			payment_date: try!(Self::get_payment_date(source.get("payment_date"))),
		})
	}
}