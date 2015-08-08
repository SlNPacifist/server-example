use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool, value_to_trait_object};
use models::VolumePayment;

#[derive(Debug, Clone)]
pub struct VolumePaymentList {
	payments: Vec<VolumePayment>
}


impl ValueAsString for VolumePayment {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for VolumePayment {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for VolumePayment {
	fn get_property(&self, name: &str) -> Option<&Value> {
		match name {
			"id" => Some(&self.id),
			"volume" => Some(&self.volume),
			"consumer_id" => Some(&self.consumer_id),
			_ => None
		}
	}
}

impl ValueAsBool for VolumePayment {
	fn as_bool(&self) -> bool {
		true
	}
}

impl ValueAsString for VolumePaymentList {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for VolumePaymentList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.payments.iter().map(value_to_trait_object)))
	} 
}

impl ValueAsObject for VolumePaymentList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for VolumePaymentList {
	fn as_bool(&self) -> bool {
		!self.payments.is_empty()
	}
}

impl From<Vec<VolumePayment>> for VolumePaymentList {
	fn from(v: Vec<VolumePayment>) -> VolumePaymentList {
		VolumePaymentList {
			payments: v
		}
	}
}