// Implementations needed for dtl

use dtl::Value;
use std::fmt;
use models::Consumer;

impl Clone for Consumer {
    fn clone(&self) -> Consumer {
        Consumer {
            id: self.id,
            address: self.address.clone()
        }
    }
}

impl fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Consumer (id: {}, address: {})", self.id, self.address)
    }
}

impl fmt::Debug for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Consumer (id: {}, address: {})", self.id, self.address)
    }
}

impl Value for Consumer {
	fn get_children(&self) -> Vec<Box<Value>> {
		Vec::new()
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ConsumerList {
	consumers: Vec<Consumer>
}

impl fmt::Display for ConsumerList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Consumer list ({} elements total)", self.consumers.len())
    }
}

impl From<Vec<Consumer>> for ConsumerList {
	fn from(v: Vec<Consumer>) -> ConsumerList {
		ConsumerList {
			consumers: v
		}
	}
}

fn box_consumer(c: &Consumer) -> Box<Value> {
	Box::new(c.clone())
}

impl Value for ConsumerList {
	fn get_children(&self) -> Vec<Box<Value>> {
		self.consumers.iter().map(box_consumer).collect()
	} 
}

