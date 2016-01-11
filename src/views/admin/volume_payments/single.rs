use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use persistent::Read;
use chrono::NaiveDate;
use db::Database;
use models::{VolumePayment, Consumer};
use views::utils::*;
use views::admin::consumers::ConsumerHandler;
use forms::*;
use super::VolumePaymentHandler;


pub fn printable(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::volume_payment::printable")
		.get().expect("Could not get connection in admin::volume_payment::printable");
	let vp = req.extensions.remove::<VolumePaymentHandler>()
		.expect("Could not get volume payment in admin::volume_payment::single::printable");
	let mut total_volume = 0.0;
	for payment in VolumePayment::up_to_date(&connection, vp.consumer_id, vp.payment_date) {
		total_volume += payment.volume;
	}
	update_var(req, "total_volume", total_volume);
	update_var(req, "consumer", Consumer::by_id(&connection, vp.consumer_id).unwrap());
	update_var(req, "payment", vp);
	render_ok(req, "admin/volume_payments/printable.htmt")
}

pub fn editable(req: &mut Request) -> IronResult<Response> {
	let vp = req.extensions.remove::<VolumePaymentHandler>()
		.expect("Could not get volume payment in admin::volume_payment::single::editable");
	update_var(req, "payment", vp);
	render_ok(req, "admin/volume_payments/single.htmt")
}

pub fn save(req: &mut Request) -> IronResult<Response> {
	let form_opt = PaymentForm::from_request(req);
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::volume_payment::single::save")
		.get().expect("Could not get connection in admin::volume_payment::single::save");
	let vp = req.extensions.get::<VolumePaymentHandler>()
		.expect("Could not get volume payment in admin::volume_payment::single::save");
	let loc = match form_opt {
		Ok(form) => {
			VolumePayment::update(
				&connection, vp.id, form.volume, form.payment_date, form.payment_sum);
			format!("/admin/consumers/{}/?payment_saved", vp.consumer_id)
		},
		_ => format!("/admin/consumers/{}/?payment_not_saved", vp.consumer_id)
	};
	redirect(loc)
}

pub fn add(req: &mut Request) -> IronResult<Response> {
	let form_opt = PaymentForm::from_request(req);
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::volume_payment::add")
		.get().expect("Could not get connection in admin::volume_payment::add");
    let consumer = req.extensions.get::<ConsumerHandler>()
    	.expect("Could not get consumer in admin::volume_payment::add");
	let loc = match form_opt {
		Ok(form) => {
			VolumePayment::insert(
				&connection, form.volume, consumer.id, form.payment_date, form.payment_sum);
			format!("/admin/consumers/{}/?payment_added", consumer.id)
		}
		Err(_) => format!("/admin/consumers/{}/?payment_not_added", consumer.id)
	};
	redirect(loc)
}

#[derive(Debug, Clone)]
struct PaymentForm {
	volume: f32,
	payment_sum: f32,
	payment_date: NaiveDate,
}

impl PaymentForm {
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
	pub fn from_request(req: &mut Request) -> Result<PaymentForm> {
		let source = try!(get_body(req));
		Ok(PaymentForm {
			volume: try!(Self::get_volume(source.get("volume"))),
			payment_sum: try!(Self::get_payment_sum(source.get("payment_sum"))),
			payment_date: try!(Self::get_payment_date(source.get("payment_date"))),
		})
	}
}
