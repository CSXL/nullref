use env_logger::Env;
use log::{info, trace};
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type Sink = SplitSink<WebSocketStream<TcpStream>, Message>;
type Stream = SplitStream<WebSocketStream<TcpStream>>;

async fn establish_websocket_handshake(tcp_stream: TcpStream) -> WebSocketStream<TcpStream> {
    let ws_stream: WebSocketStream<TcpStream> = tokio_tungstenite::accept_async(tcp_stream)
        .await
        .expect("Error during the websocket handshake");
    return ws_stream;
}

fn create_mpsc_channel() -> (UnboundedSender<Message>, UnboundedReceiver<Message>) {
    let (tx, rx) = unbounded();
    return (tx, rx);
}

fn add_peer_to_map(
    address: SocketAddr,
    transmitting_channel: UnboundedSender<Message>,
    peer_map: &PeerMap,
) {
    peer_map
        .lock()
        .unwrap()
        .insert(address, transmitting_channel);
}

fn split_websocket_stream(websocket_stream: WebSocketStream<TcpStream>) -> (Sink, Stream) {
    let (sink, stream) = websocket_stream.split();
    return (sink, stream);
}

fn broadcast_to_all(message: Message, peer_map: &PeerMap) -> Result<(), tungstenite::Error> {
    let peers = peer_map.lock().unwrap();
    let peer_transmission_channels = peers.iter().map(|(_, ws_sink)| ws_sink);
    for channel in peer_transmission_channels {
        channel.unbounded_send(message.clone()).unwrap();
    }

    Ok(())
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, peer_address: SocketAddr) {
    info!("Incoming TCP connection from: {}", peer_address);

    trace!("Initiating websocket handshake");
    let ws_stream = establish_websocket_handshake(raw_stream).await;
    info!("Websocket connection established with {}", peer_address);

    let (tx, rx) = create_mpsc_channel();
    add_peer_to_map(peer_address.clone(), tx.clone(), &peer_map);

    let (outgoing, incoming) = split_websocket_stream(ws_stream);
    let handle_incoming = incoming.try_for_each(|message: Message| {
        info!(
            "Received a message from {}: {}",
            peer_address,
            message.to_text().unwrap()
        );
        broadcast_to_all(message, &peer_map).expect("Error broadcasting message.");
        future::ok(())
    });
    let handle_outgoing = rx.map(Ok).forward(outgoing);

    pin_mut!(handle_incoming, handle_outgoing);
    future::select(handle_incoming, handle_outgoing).await;

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
