mod server;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_server() {
        let server: server::Server = Server::new();
    }
}
