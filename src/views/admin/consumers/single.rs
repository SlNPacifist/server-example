use std::io::Result;
use iron::prelude::*;
use persistent::Read;
use chrono;
use db::Database;
use models::{VolumePayment, Consumer};
use dtl_impls::VolumePaymentList;
use super::ConsumerHandler;
use views::utils::*;
use forms::*;


pub fn entry(req: &mut Request) -> IronResult<Response> {
    let connection = req.get::<Read<Database>>()
        .expect("Could not get connection pool in admin::consumers::single::entry")
        .get().expect("Could not get connection in admin::consumers::single::entry");
    let consumer = req.extensions.remove::<ConsumerHandler>()
        .expect("Could not get consumer in admin::consumers::single::entry");
    let payments = VolumePayment::for_consumer(&connection, consumer.id);
    let mut volume_sum = 0.0;
    let mut money_sum = 0.0;
    for p in payments.iter() {
        volume_sum += p.volume;
        money_sum += p.sum;
    }
    update_var(req, "consumer", consumer);
    update_var(req, "payments", VolumePaymentList::new(payments)); 
    update_var(req, "total_volume_sum", volume_sum); 
    update_var(req, "total_money_sum", money_sum); 
    update_var(req, "today", chrono::Local::today()); 
    render_ok(req, "admin/consumers/single.htmt")
}

pub fn add_consumer(req: &mut Request) -> IronResult<Response> {
    let form_opt = ConsumerForm::from_request(req);
    let connection = req.get::<Read<Database>>()
        .expect("Could not get connection pool in admin::consumer::single::add_consumer")
        .get().expect("Could not get connection in admin::consumer::single::add_consumer");
    let loc = match form_opt {
        Ok(form) => {
            Consumer::insert(&connection, form.address, form.phone, form.name);
            "/admin/consumers/?consumer_added"
        }
        Err(_) => "/admin/consumers/?consumer_not_added"
    };
    redirect(loc.to_string())
}

pub fn update_consumer(req: &mut Request) -> IronResult<Response> {
    let form_opt = ConsumerForm::from_request(req);
    let consumer = req.extensions.remove::<ConsumerHandler>()
        .expect("Could not get consumer in admin::consumers::single::update_consumer");
    let connection = req.get::<Read<Database>>()
        .expect("Could not get connection pool in admin::consumer::single::update_consumer")
        .get().expect("Could not get connection in admin::consumer::single::update_consumer");
    let loc = match form_opt {
        Ok(form) => {
            Consumer::update(&connection, consumer.id, form.address, form.phone, form.name);
            format!("/admin/consumers/{}/?consumer_saved", consumer.id)
        }
        Err(_) => format!("/admin/consumers/{}/?consumer_not_saved", consumer.id)
    };
    redirect(loc)
}

#[derive(Debug, Clone)]
struct ConsumerForm {
    address: String,
    name: String,
    phone: String,
}

impl ConsumerForm {
    pub fn from_request(req: &mut Request) -> Result<ConsumerForm> {
        let body = try!(get_body(req));
        Ok(ConsumerForm {
            address: try!(parse_single_field(body.get("address"), "address")).to_string(),
            name: try!(parse_single_field(body.get("name"), "address")).to_string(),
            phone: try!(parse_single_field(body.get("phone"), "address")).to_string(),
        })
    }
}
