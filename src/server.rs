const DEFAULT_ADDRESS: &str = "127.0.0.1:8080";

mod server {
    use futures_util::future;
    use futures_util::future::Ready;
    use tokio::net::TcpListener;
    use tungstenite::Error;

    #[derive(Debug)]
    pub struct Server {
        pub address: Option<String>,
        pub(crate) listener: Option<TcpListener>,
    }

    impl Server {
        pub fn new() -> Self {
            Self {
                address: None,
                listener: None,
            }
        }

        pub async fn start(&mut self, address: &str) -> Result<(), Error> {
            let address = String::from(address);
            self.address = Some(address.clone());
            self.bind(address).await;
            Ok(())
        }

        async fn bind(&mut self, address: String) -> &Server {
            let try_socket = TcpListener::bind(&address).await;
            let listener = try_socket.expect("Failed to bind to socket");
            self.listener = Some(listener);
            return self;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use server::Server;
    use std::any::type_name;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn start_server() {
        let mut server = Server::new();
        server
            .start(DEFAULT_ADDRESS)
            .await
            .expect("Server failed to start.");
        let server_address = server.address.as_ref().unwrap();
        assert_eq!(server_address, DEFAULT_ADDRESS);
        if let None = server.listener {
            panic!("Server listener not initialized.");
        }
    }
}
