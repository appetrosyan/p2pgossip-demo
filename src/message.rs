use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Message {
	pub message: String,
}

impl std::fmt::Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "p2pgossip message: {}", self.message)
	}
}

impl From<String> for Message {
	fn from(other_message: String) -> Message {
		Message {
			message: other_message,
		}
	}
}

impl From<&str> for Message {
	fn from(other_message: &str) -> Message {
		Message {
			message: String::from(other_message),
		}
	}
}
