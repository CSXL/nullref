use crate::client::Client;
mod client;
mod message;

fn main() {
    let mut client = Client::new("0.0.0.0:8080");
    client.start();
    // Wait indefinitely
    loop {
        let message = client.rx.recv().unwrap();
        println!("Incoming Message: {}", message);
    }
}