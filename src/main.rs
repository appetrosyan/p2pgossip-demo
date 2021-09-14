use chrono::{DateTime, Utc};
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{net::{Ipv4Addr, SocketAddr, ToSocketAddrs}, time::Duration, vec::Vec};
use actix_web::{HttpServer, Responder};

#[derive(Serialize, Deserialize, Debug)]
struct Peer {
	started: DateTime<Utc>,
	period: std::time::Duration,
	host: Option<String>,
	port: u16
}

struct ApplicationState {
	known_peers: Vec<Peer>,
}

#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = App::new("p2pGossip")
		.author("Aleksandr Petrosyan")
		.arg(
			Arg::with_name("port")
				.long("port")
				.help("The port to start the peer listening on")
				.short("p")
				.takes_value(true)
				.required(true),
		)
		.arg(
			Arg::with_name("connect")
				.long("connect")
				.short("c")
				.visible_aliases(&["connect-to", "make-connection"])
				.help("The first peer to connect to")
				.takes_value(true)
				.multiple(true)
				.required(false),
		)
		.arg(
			Arg::with_name("period")
				.long("period")
				.short("P")
				.visible_aliases(&["message_interval", "message_period"])
				.default_value("5"),
		)
		.get_matches();

	let this_peer = this_peer_from_args(args).unwrap();
	HttpServer::new(|| {actix_web::App::new()})
		.bind((Ipv4Addr::LOCALHOST, *&this_peer.port))?
	.run()
		.await?;
	Ok(())
}

fn this_peer_from_args(args: clap::ArgMatches) -> Result<Peer, Box<dyn std::error::Error>> {
	let started = DateTime::<Utc>::from(std::time::SystemTime::now());
	let period: Duration =
		Duration::from_secs(args.value_of("period").unwrap_or_default().parse::<u64>()?);
	let port = args.value_of("port").unwrap().parse::<u16>()?;
	let this_peer = Peer {
		started,
		period,
		host: None,
		port
	};
	Ok(this_peer)
}
