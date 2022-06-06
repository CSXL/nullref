#[allow(clippy::module_inception)]
mod server {
    use tokio::net::TcpListener;
    use url::Url;

    #[derive(Debug)]
    pub struct Server {
        // Fields are options so they can temporarily be None
        pub address: Option<String>,
        pub(crate) listener: Option<TcpListener>,
    }

    impl Server {
        #[allow(dead_code)] // Remove once rest of module is implemented
        pub fn new() -> Self {
            Self {
                address: None,
                listener: None,
            }
        }

        #[allow(dead_code)] // Remove once rest of module is implemented
        pub async fn start<'a>(&mut self, address: &'a str) -> Result<(), &'a str> {
            let result = Self::validate_address(address);
            if result.is_err() {
                let error = result.err().unwrap();
                return Err(error);
            }
            let address = String::from(address);
            self.address = Some(address.clone());
            self.bind(address).await;
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
        server
            .start(DEFAULT_ADDRESS)
            .await
            .expect("Server failed to start.");
        let server_address = server
            .address
            .as_ref()
            .unwrap_or_else(|| panic!("Server has no address"));
        assert_eq!(server_address, DEFAULT_ADDRESS);
        if let None = server.listener {
            panic!("Server listener not initialized.");
        }
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
}
