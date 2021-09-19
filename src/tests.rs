#[cfg(test)]
pub mod tests {
	use std::{collections::HashMap, net::SocketAddr, sync::Arc};

	use chrono::{DateTime, Utc};
	use std::sync::Mutex;

	use crate::{application_state::ApplicationState, message::Message, peer::Peer};

	fn default_peer() -> Peer {
		Peer {
			started: DateTime::<Utc>::from(std::time::SystemTime::now()),
			period: std::time::Duration::from_secs(5),
			port: 8080,
			host_alias: None,
		}
	}

	fn setup_app_state() -> Arc<ApplicationState> {
		Arc::new(ApplicationState {
			known_peers: Mutex::new(HashMap::<SocketAddr, Peer>::new()),
			this_peer: default_peer(), //
			keep_running: true.into(),
			update_known_peers: true.into(), // We want to test if it works, right?
		})
	}

	#[actix_rt::test]
	async fn index() {
		let app_state = setup_app_state();
		let mut server = actix_web::test::init_service(
			actix_web::App::new()
				.app_data(actix_web::web::Data::new(app_state.clone()))
				.service(crate::server::get_protocol),
		)
		.await;
		let req_get = actix_web::test::TestRequest::get().uri("/").to_request();
		let resp = actix_web::test::call_service(&mut server, req_get).await;
		assert!(resp.status().is_success());

		let req_get = actix_web::test::TestRequest::get().uri("/").to_request();
		let resp: Peer = actix_web::test::read_response_json(&mut server, req_get).await;
		assert_eq!(resp.port, 8080);

		let req_post = actix_web::test::TestRequest::post().uri("/").to_request();
		let resp = actix_web::test::call_service(&mut server, req_post).await;
		assert!(!resp.status().is_success());
	}

	fn default_message() -> Message {
		Message {
			message: String::from("Hello, world"),
		}
	}

	#[actix_rt::test]
	async fn connecting() {
		let app_state = setup_app_state();
		let mut server = actix_web::test::init_service(
			actix_web::App::new()
				.app_data(actix_web::web::Data::new(app_state.clone()))
				.service(crate::server::connect),
		)
		.await;
		let req_get = actix_web::test::TestRequest::get()
			.uri("/connect")
			.to_request();
		let resp = actix_web::test::call_service(&mut server, req_get).await;
		assert!(!resp.status().is_success());
		let req = actix_web::test::TestRequest::post()
			.uri("/connect")
			.set_json(&default_peer())
			.to_request();
		let resp = actix_web::test::call_service(&mut server, req).await;
		assert!(resp.status().is_success());
	}

	#[actix_rt::test]
	async fn messaging() {
		let app_state = setup_app_state();
		let mut server = actix_web::test::init_service(
			actix_web::App::new()
				.app_data(actix_web::web::Data::new(app_state.clone()))
				.service(crate::server::message),
		)
		.await;
		let req_get = actix_web::test::TestRequest::get()
			.uri("/message")
			.to_request();
		let resp = actix_web::test::call_service(&mut server, req_get).await;
		assert!(!resp.status().is_success());

		let req = actix_web::test::TestRequest::post()
			.uri("/message")
			.set_json(&default_message())
			.to_request();
		let resp: Message = actix_web::test::read_response_json(&mut server, req).await;
		assert_eq!(resp.message, "Hello, world".to_string());
	}

	#[actix_rt::test]
	async fn zero_initial_known_peers() {
		let app_state = setup_app_state();
		let mut server = actix_web::test::init_service(
			actix_web::App::new()
				.app_data(actix_web::web::Data::new(app_state.clone()))
				.service(crate::server::get_known_peers)
		)
		.await;
		let req_get = actix_web::test::TestRequest::get()
			.uri("/known_peers")
			.to_request();
		let resp: HashMap<SocketAddr, Peer> =
			actix_web::test::read_response_json(&mut server, req_get).await;
		assert!(resp.is_empty());
	}

	#[actix_rt::test]
	async fn connecting_adds_this_peer() {
		let mut this_peer = default_peer();
		this_peer.port = 8081;
		let app_state = setup_app_state();
		let mut server = actix_web::test::init_service(
			actix_web::App::new()
				.app_data(actix_web::web::Data::new(app_state.clone()))
				.service(crate::server::connect)
		)
		.await;

		let req_connect = actix_web::test::TestRequest::post()
			.uri("/connect")
			.set_json(&this_peer)
			.to_request();
		actix_web::test::call_service(&mut server, req_connect).await;
		assert!(app_state.known_peers.lock().unwrap().contains_key(&this_peer.into()));
	}
}
