#[allow(clippy::module_inception)]
mod server {
    use env_logger::Env;
    use log::{info, trace};
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use futures_channel::mpsc::UnboundedSender;
    use tokio::net::TcpListener;
    use tungstenite::Message;
    use url::Url;

    type Tx = UnboundedSender<Message>;
    type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

    #[derive(Debug)]
    pub struct Server {
        // Fields are options so they can temporarily be None
        pub address: Option<String>,
        pub(crate) listener: Option<TcpListener>,
        pub(crate) connections: Option<PeerMap>,
    }

    impl Server {
        #[allow(dead_code)] // Remove once rest of module is implemented
        pub fn new() -> Self {
            Self {
                address: None,
                listener: None,
                connections: None,
            }
        }

        #[allow(dead_code)] // Remove once rest of module is implemented
        pub async fn start<'a>(&mut self, address: &'a str) -> Result<(), &'a str> {
            info!("Starting server.");
            let result = Self::validate_address(address);
            if result.is_err() {
                let error = result.err().unwrap();
                return Err(error);
            }
            let address = String::from(address);
            self.address = Some(address.clone());
            trace!("Binding to address provided.");
            self.bind(address).await;
            trace!("Binding successful.");
            info!("Server started.");
            Ok(())
        }

        #[allow(dead_code)] // Remove once rest of module is implemented
        fn validate_address(address: &str) -> Result<(), &str> {
            let url_parse_result = Url::parse(&*("https://".to_owned() + address));
            if url_parse_result.is_err() {
                return Err(
                    "Invalid address passed to server.start. Example valid address: \
                127.0.0.1:8080",
                );
            }

            let url = url_parse_result.unwrap();
            if url.port() == None {
                return Err("Port not specified in address. Example valid address: 127.0.0.1:8080");
            } else if url.port().unwrap() < 1024 {
                return Err(
                    "Port numbers below 1024 (excluding special cases) are privileged and \
                non-writable by users. More information: \
                https://www.w3.org/Daemon/User/Installation/PrivilegedPorts.html",
                );
            }
            Ok(())
        }

        #[allow(dead_code)] // Remove once rest of module is implemented
        async fn bind(&mut self, address: String) -> &Server {
            let try_socket = TcpListener::bind(&address).await;
            let listener = try_socket.expect("Failed to bind to socket");
            self.listener = Some(listener);
            self
        }

        pub async fn accept_connections(&mut self) {
            self.connections = Some(
                PeerMap::new(Mutex::new(HashMap::new()))
            );
            tokio::spawn(async {
               while let Ok((tcp_stream, address)) = self.listener.unwrap().accept().await {
                   tokio::spawn(self.handle_connection(tcp_stream, address));
               }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    const DEFAULT_ADDRESS: &str = "127.0.0.1:8080";
    use super::*;
    use server::Server;

    #[tokio::test]
    async fn start_server() {
        let mut server = Server::new();
        _start_server(&mut server).await;
        let server_address = server
            .address
            .as_ref()
            .unwrap_or_else(|| panic!("Server has no address"));
        assert_eq!(server_address, DEFAULT_ADDRESS);
        assert_ne!(None, server.listener, "Server listener not initialized.");
        server.accept_connections().await;
        assert_ne!(None, server.connections, "Server connections map not initialized.");
    }

    async fn _start_server(server: &mut Server) {
        server
            .start(DEFAULT_ADDRESS)
            .await
            .expect("Server failed to start.");
    }

    #[tokio::test]
    async fn start_server_with_bad_addresses() {
        let mut server = Server::new();
        let invalid_address = get_error_on_start(
            &mut server,
            "This is a very invalid \
        address.",
        )
        .await;
        assert_eq!(
            invalid_address,
            "Invalid address passed to server.start. \
        Example valid address: 127.0.0.1:8080"
        );
        let host_without_port = get_error_on_start(&mut server, "127.0.0.1").await;
        assert_eq!(
            host_without_port,
            "Port not specified in address. \
        Example valid address: 127.0.0.1:8080"
        );
        let port_below_1024 = get_error_on_start(&mut server, "127.0.0.1:1001").await;
        assert_eq!(
            port_below_1024,
            "Port numbers below 1024 (excluding special cases) are \
        privileged and non-writable by users. More information: \
        https://www.w3.org/Daemon/User/Installation/PrivilegedPorts.html"
        );
    }

    async fn get_error_on_start(server: &mut Server, address: &str) -> String {
        let result = server.start(address).await.unwrap_err().to_string();
        return result;
    }

    #[tokio::test]
    async fn start_server_and_listen() {

    }
}
