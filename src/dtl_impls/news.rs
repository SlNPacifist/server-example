use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool, value_to_trait_object};
use models::News;

#[derive(Debug, Clone)]
pub struct NewsList(Vec<News>);


impl ValueAsString for News {
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ValueAsIterator for News {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for News {
	fn get_property(&self, name: &str) -> Option<&Value> {
		match name {
			"header" => Some(&self.header),
			"text" => Some(&self.text),
			"id" => Some(&self.id),
			"publication_date" => Some(&self.publication_date),
			_ => None,
		}
	}
}

impl ValueAsBool for News {
	fn as_bool(&self) -> bool {
		true
	}
}


impl ValueAsString for NewsList {
    fn as_string(&self) -> String {
        format!("News list ({} elements total)", self.0.len())
    }
}

impl ValueAsIterator for NewsList {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.0.iter().map(value_to_trait_object)))
	}
}

impl ValueAsObject for NewsList {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for NewsList {
	fn as_bool(&self) -> bool {
		!self.0.is_empty()
	}
}

impl NewsList {
	pub fn new(v: Vec<News>) -> NewsList {
		NewsList(v)
	}
}