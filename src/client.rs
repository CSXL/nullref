//! Represents a client that accept and connect to other clients.
//! All the objects and methods are testable without other running peers.
//! There should also be an interactive repl for the client to use.
//! This should only use standard libraries.
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::message::{build_discovery, build_error, build_message};

/// A client that can connect to other clients.
/// This is the main object that should be used to interact with the network.
pub struct Client {
    /// The address of the client.
    pub addr: SocketAddr,
    /// The listener for the client.
    pub listener: TcpListener,
    /// The list of peers that the client is connected to.
    pub peers: HashMap<SocketAddr, TcpStream>,
    /// The channel to send messages to the client.
    pub tx: Sender<String>,
    /// The channel to receive messages from the client.
    pub rx: Receiver<String>,
}

impl Client {
    /// Creates a new client.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::client::Client;
    ///
    /// let client = Client::new("http://127.0.0.1:8080");
    /// ```
    pub fn new(addr: &str) -> Client {
        let addr = addr.parse().unwrap();
        let listener = TcpListener::bind(addr).unwrap();
        let (tx, rx) = channel();
        Client {
            addr,
            listener,
            peers: HashMap::new(),
            tx,
            rx,
        }
    }

    /// Starts the client and listens for incoming connections.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::client::Client;
    ///
    /// let client = Client::new("http://127.0.0.1:8080");
    /// client.start();
    /// ```
    pub fn start(&mut self) {
        let listener = self.listener.try_clone().unwrap();
        let tx = self.tx.clone();
        let client_addr = self.addr;
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let addr = stream.peer_addr().unwrap();
                        let message = build_discovery(
                            addr.to_string(),
                            client_addr.to_string().to_string(),
                            "".to_string(),
                        );
                        println!("Outgoing Message: {}", message);
                        tx.send(message).unwrap();
                    }
                    Err(e) => {
                        let message = build_error(
                            client_addr.to_string(),
                            client_addr.to_string(),
                            e.to_string(),
                        );
                        tx.send(message).unwrap();
                    }
                }
            }
        });
    }
}
