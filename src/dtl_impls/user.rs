use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool, value_to_trait_object};
use models::User;

#[derive(Debug, Clone)]
pub struct UserList(pub Vec<User>);


impl ValueAsString for User {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for User {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for User {
	fn get_property(&self, name: &str) -> Option<&Value> {
		match name {
			"login" => Some(&self.login),
			"consumer_id" => self.consumer_id.as_ref().map(|id| id as &Value),
			"consumer" => self.consumer.as_ref().map(|c| c as &Value),
			_ => None
		}
	}
}

impl ValueAsBool for User {
	fn as_bool(&self) -> bool {
		true
	}
}


impl ValueAsString for UserList {
    fn as_string(&self) -> String {
        format!("User list ({} elements total)", self.0.len())
    }
}

impl ValueAsIterator for UserList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.0.iter().map(value_to_trait_object)))
	} 
}

impl ValueAsObject for UserList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for UserList {
	fn as_bool(&self) -> bool {
		!self.0.is_empty()
	}
}