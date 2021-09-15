use std::{error::Error, net::SocketAddr};

use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Peer {
	pub started: DateTime<Utc>,
	pub period: Option<std::time::Duration>,
	pub port: u16,
}

impl From<SocketAddr> for Peer {
	fn from(addr: SocketAddr) -> Self {
		Peer {
			started: DateTime::<Utc>::from(std::time::SystemTime::now()),
			period: None,
			port: addr.port(),
		}
	}
}

impl Peer {
	pub fn try_from(args: clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
		let started = DateTime::<Utc>::from(std::time::SystemTime::now());
		let period =
			Duration::from_secs(args.value_of("period").unwrap_or_default().parse::<u64>()?);
		let port = args.value_of("port").unwrap().parse::<u16>()?;
		Ok(Peer {
			started,
			period: Some(period),
			port,
		})
	}
}
