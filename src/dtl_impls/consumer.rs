use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool, value_to_trait_object};
use chrono::NaiveDate;
use models::Consumer;

#[derive(Debug, Clone)]
pub struct ConsumerList(pub Vec<Consumer>);

#[derive(Debug, Clone)]
pub struct ConsumerWithPaymentInfo {
	consumer: Consumer,
	total_volume: f32,
	last_payment_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct ConsumerWithPaymentInfoList(pub Vec<ConsumerWithPaymentInfo>);


impl ValueAsString for Consumer {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for Consumer {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for Consumer {
	fn get_property(&self, name: &str) -> Option<&Value> {
		match name {
			"id" => Some(&self.id),
			"address" => Some(&self.address),
			"name" => Some(&self.name),
			"phone" => Some(&self.phone),
			_ => None
		}
	}
}

impl ValueAsBool for Consumer {
	fn as_bool(&self) -> bool {
		true
	}
}


impl ValueAsString for ConsumerList {
    fn as_string(&self) -> String {
        format!("Consumer list ({} elements total)", self.0.len())
    }
}

impl ValueAsIterator for ConsumerList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.0.iter().map(value_to_trait_object)))
	} 
}

impl ValueAsObject for ConsumerList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for ConsumerList {
	fn as_bool(&self) -> bool {
		!self.0.is_empty()
	}
}


impl ValueAsString for ConsumerWithPaymentInfo {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for ConsumerWithPaymentInfo {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for ConsumerWithPaymentInfo {
	fn get_property(&self, name: &str) -> Option<&Value> {
		match name {
			"consumer" => Some(&self.consumer),
			"total_volume" => Some(&self.total_volume),
			"last_payment_date" => self.last_payment_date.as_ref().map(value_to_trait_object),
			_ => None
		}
	}
}

impl ValueAsBool for ConsumerWithPaymentInfo {
	fn as_bool(&self) -> bool {
		true
	}
}


impl ValueAsString for ConsumerWithPaymentInfoList {
    fn as_string(&self) -> String {
        format!("Consumer with payment list ({} elements total)", self.0.len())
    }
}

impl ValueAsIterator for ConsumerWithPaymentInfoList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.0.iter().map(value_to_trait_object)))
	} 
}

impl ValueAsObject for ConsumerWithPaymentInfoList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for ConsumerWithPaymentInfoList {
	fn as_bool(&self) -> bool {
		!self.0.is_empty()
	}
}

impl ConsumerWithPaymentInfoList {
	pub fn new(v: Vec<(Consumer, f32, Option<NaiveDate>)>) -> ConsumerWithPaymentInfoList {
		ConsumerWithPaymentInfoList (
			v.into_iter().map(|d| {
				let (a, b, c) = d;
				ConsumerWithPaymentInfo{ consumer: a, total_volume: b, last_payment_date: c }
			}).collect()
		)
	}
}
