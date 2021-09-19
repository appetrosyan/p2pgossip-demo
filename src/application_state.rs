use std::{
	collections::HashMap,
	net::SocketAddr,
	sync::{atomic::AtomicBool, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::peer::Peer;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationState {
	pub known_peers: Mutex<HashMap<SocketAddr, Peer>>,
	pub this_peer: Peer,
	pub keep_running: AtomicBool,
	pub update_known_peers: AtomicBool,
}

impl ApplicationState {
	pub fn add_known_peer(&self, socket_addr: SocketAddr, peer: Peer) {
		self.known_peers.lock().unwrap().insert(socket_addr, peer);
	}

	pub fn known_peers(&self) -> HashMap<SocketAddr, Peer> {
		self.known_peers.lock().unwrap().clone()
	}
}
