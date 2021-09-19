use std::{
	collections::HashMap,
	error::Error,
	net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
	sync::{atomic::Ordering::Relaxed, Arc},
};

use actix_web::web::Json;
use chrono::{DateTime, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{application_state::ApplicationState, message::Message};
use crate::logging::LogExt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Peer {
	pub started: DateTime<Utc>,
	pub period: std::time::Duration,
	pub port: u16,
	pub host_alias: Option<IpAddr>,
}

impl From<Json<Peer>> for Peer {
	fn from(json: Json<Peer>) -> Self {
		Peer {
			started: json.started,
			period: json.period,
			port: json.port,
			host_alias: json.host_alias,
		}
	}
}

impl Into<SocketAddr> for Peer {
	fn into(self) -> SocketAddr {
		let addr: IpAddr = match self.host_alias {
			None => std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
			Some(addr) => addr,
		};
		(addr, self.port).into()
	}
}

impl ToSocketAddrs for Peer {
	type Iter = std::option::IntoIter<SocketAddr>;

	fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
		let socket: SocketAddr = (*self).into();
		socket.to_socket_addrs()
	}
}

impl Peer {
	pub fn try_from(args: &clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
		let started = DateTime::<Utc>::from(std::time::SystemTime::now());
		let period =
			Duration::from_secs(args.value_of("period").unwrap_or_default().parse::<u64>().log()?);
		let port = args.value_of("port").unwrap().parse::<u16>().log()?;
		let host_alias = match args.value_of("host_alias") {
			Some(value) => Some(value.parse().log()?),
			None => None,
		};

		Ok(Peer {
			started,
			period,
			port,
			host_alias,
		})
	}
}

pub(crate) async fn check_if_peer(peer: SocketAddr) -> Result<Peer, Box<dyn Error>> {
	info!("Checking if {} is a p2pGossip instance", peer);
	let client = actix_web::client::Client::default();
	let mut asn = client.get(format!("http://{}", &peer)).send().await.log()?;
	let reply = asn.json::<Peer>().await;
	info!("Got {:?}", reply);
	Ok(reply?)
}

pub(crate) async fn try_connect(
	addr: SocketAddr,
	peer: Peer,
	app_state: Arc<ApplicationState>,
) -> Result<(), Box<dyn Error>> {
	info!("Connecting to {}", addr);
	let client = actix_web::client::Client::default();
	let this_peer = app_state.this_peer;
	let mut response = client
		.post(format!("http://{}/connect", addr))
		.send_json(&this_peer)
		.await.log()?;
	let reply = response.body().await.log()?;
	info!("Got {:?}, from {}", reply, addr);
	Ok(app_state.add_known_peer(addr, peer))
}

pub(crate) async fn discover_other_peers(
	addr: &SocketAddr,
) -> Result<HashMap<SocketAddr, Peer>, Box<dyn Error>> {
	info!("Finding peers known to {}", addr);
	let client = actix_web::client::Client::default();
	let mut response = client
		.get(format!("http://{}/known_peers", addr))
		.send()
		.await.log()?;
	let reply = response.json::<HashMap<SocketAddr, Peer>>().await.log()?;
	Ok(reply)
}

async fn message_peer(
	addr: &SocketAddr,
	msg: impl Into<Message>,
) -> Result<SocketAddr, Box<dyn Error>> {
	let message = msg.into();
	let client = actix_web::client::Client::default();
	let mut response = client
		.post(format!("http://{}/message", addr))
		.send_json(&message)
		.await.log()?;
	response.body().await.log()?;
	Ok(*addr)
}

pub(crate) async fn update_known_peers(
	app_state: Arc<ApplicationState>,
	known_peer_sockets: &Vec<SocketAddr>,
) -> () {
	// Getting all known peers might take a while.
	// So this has to be done concurrently.
	let peer_maps: Vec<_> = known_peer_sockets
		.iter()
		.map(|addr| discover_other_peers(addr))
		.collect();
	let peer_maps = futures::future::join_all(peer_maps).await;
	for map in peer_maps{
		match map {
			Ok(mut val) => {
				val.remove(&app_state.this_peer.into());
				let mut global_peers = app_state.known_peers.lock().unwrap();
				global_peers.extend(val);
			} _ => ()
		}
	}
	info!("Fetched known peers");
}

pub(crate) async fn periodically_send_message(
	message: &str,
	app_state: Arc<ApplicationState>,
) -> Result<(), Box<dyn Error>> {
	let period = app_state.this_peer.period;

	let mut interval = tokio::time::interval(period);
	while app_state.keep_running.load(Relaxed) {
		interval.tick().await;

		info!("Time to message");
		let known_peer_sockets: Vec<SocketAddr> = app_state
			.known_peers()
			.keys()
			.into_iter()
			.map(|&key| (key).clone()) // Make the borrow-checker happy.
			.collect();

		// TODO make the two processes concurrent.

		if app_state.update_known_peers.load(Relaxed) {
			update_known_peers(app_state.clone(), &known_peer_sockets).await;
		}

		info!("messaging {:?}", known_peer_sockets);
		//  Post requests are quick, and not awaiting on a collection is easier to read.
		for key in &known_peer_sockets {
			if let Err(err) = message_peer(&key, message).await {
				debug!("{}", err);
				info!("removing {} because of {}", key, err);
				let mut global_peers = app_state.known_peers.lock().unwrap();
				global_peers.remove(&key);
			}
		}
	}
	Ok(())
}
