/*
Objective: Create a full duplex connection between a client and server.
*/

use env_logger::Env;
use log::{info, trace};
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn establish_websocket_handshake(tcp_stream: TcpStream) -> WebSocketStream<TcpStream> {
    let ws_stream: WebSocketStream<TcpStream> = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake");
    return ws_stream;
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, peer_address: SocketAddr) {
    info!("Incoming TCP connection from: {}", peer_address);

    trace!("Initiating websocket handshake");
    let ws_stream = establish_websocket_handshake(raw_stream).await;
    info!("Websocket connection established with {}", peer_address);

    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(peer_address, tx);

    let (peer_sink, peer_stream) = ws_stream.split();

    let broadcast_incoming = peer_stream.try_for_each(|message: Message| {
        info!(
            "Received a message from {}: {}",
            peer_address,
            message.to_text().unwrap()
        );
        let locked_peers = peer_map.lock().unwrap();

        let broadcast_recepeints = locked_peers
            .iter()
            .filter(|(address, _)| address != &&peer_address)
            .map(|(_, ws_sink)| ws_sink);

        for recipient in broadcast_recepeints {
            recipient.unbounded_send(message.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_peers = rx.map(Ok).forward(peer_sink);

    pin_mut!(broadcast_incoming, receive_from_peers);
    future::select(broadcast_incoming, receive_from_peers).await;

    info!("{} disconnected", &peer_address);
    peer_map.lock().unwrap().remove(&peer_address);
    trace!("{} removed from peer map", &peer_address);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host_address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = TcpListener::bind(&host_address).await;
    let listener = try_socket.expect("Failed to bind to socket");
    info!("Listening on: {}", host_address);

    while let Ok((stream, peer_address)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, peer_address));
    }

    Ok(())
}
