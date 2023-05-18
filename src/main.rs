use dotenv;
use websocket::client::builder::ClientBuilder;
use std::env;

const AUTH_ADDRESS: &str =  "wss://wirepas-dev.peercode.nl:8811";
const REALTIME_ADDRESS: &str = "wss://wirepas-dev.peercode.nl:8813";

fn main() {
    dotenv::from_filename("credentials.env").ok().unwrap();


    let client = ClientBuilder::new(AUTH_ADDRESS);
}
