use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool, value_to_trait_object};
use models::Consumer;

#[derive(Debug, Clone)]
pub struct ConsumerList {
	consumers: Vec<Consumer>
}


impl ValueAsString for Consumer {
    fn as_string(&self) -> String {
        format!("Consumer (id: {}, address: {})", self.id, self.address)
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
        format!("Consumer list ({} elements total)", self.consumers.len())
    }
}

impl ValueAsIterator for ConsumerList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.consumers.iter().map(value_to_trait_object)))
	} 
}

impl ValueAsObject for ConsumerList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for ConsumerList {
	fn as_bool(&self) -> bool {
		!self.consumers.is_empty()
	}
}

impl From<Vec<Consumer>> for ConsumerList {
	fn from(v: Vec<Consumer>) -> ConsumerList {
		ConsumerList {
			consumers: v
		}
	}
}