#[allow(clippy::module_inception)]
mod server {
    use futures_channel::mpsc::UnboundedSender;
    use log::{info, trace};
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::task::JoinHandle;
    use tungstenite::Message;
    use url::Url;

    type Tx = UnboundedSender<Message>;
    pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

    #[derive(Debug)]
    pub struct Server {
        // Fields are options so they can temporarily be None
        pub address: Option<String>,
        accept_connections_task: Option<JoinHandle<()>>, // to allow closing the server listener
    }

    impl Server {
        #[allow(dead_code)] // Remove once rest of module is implemented
        pub fn new() -> Self {
            Self {
                address: None,
                accept_connections_task: None,
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
            let listener = self.bind(address).await;
            trace!("Binding successful.");
            info!("Server started.");
            let accept_connections_task = tokio::spawn(async {
                Self::accept_connections(listener).await;
            });
            self.accept_connections_task = Some(accept_connections_task);
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
        async fn bind(&mut self, address: String) -> TcpListener {
            let try_socket = TcpListener::bind(&address).await;
            let listener = try_socket.expect("Failed to bind to socket");
            listener
        }

        pub async fn accept_connections(listener: TcpListener) {
            let connections = PeerMap::new(Mutex::new(HashMap::new()));
            while let Ok((tcp_stream, address)) = listener.accept().await {
                tokio::spawn(Self::handle_connection(
                    connections.clone(),
                    tcp_stream,
                    address,
                ));
            }
        }

        async fn handle_connection(
            connections: PeerMap,
            tcp_stream: TcpStream,
            address: SocketAddr,
        ) {
            // TODO: Write logic for handling connections
        }

        pub async fn close(&mut self) {
            if self.accept_connections_task.is_some() {
                self.accept_connections_task.as_ref().unwrap().abort();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const DEFAULT_HOST: &str = "127.0.0.1";

    use super::*;
    use rand as random;
    use rand::Rng as random_number_generator;
    use server::Server;

    fn generate_random_address() -> String {
        let random_number = random::thread_rng().gen_range(1024..9999); // See https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers
        let address = format!("{}:{}", DEFAULT_HOST, random_number);
        address
    }

    #[tokio::test]
    async fn start_server() {
        let mut server = Server::new();
        _start_server(&mut server).await;
        let server_address = server
            .address
            .as_ref()
            .unwrap_or_else(|| panic!("Server has no address"));
        server.close().await;
    }

    async fn _start_server(server: &mut Server) {
        server
            .start(generate_random_address().as_str())
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
    async fn server_responds_to_client() {
        let mut server = Server::new();
        _start_server(&mut server).await;
        // TODO: Integrate with mockclient after mocklient is implemented.
        server.close().await;
    }
}
