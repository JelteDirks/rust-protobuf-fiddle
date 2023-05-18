use dotenv;
use serde_json::json;
use std::env;

const AUTH_ADDRESS: &str = "wss://wirepas-dev.peercode.nl:8811";
const REALTIME_ADDRESS: &str = "wss://wirepas-dev.peercode.nl:8813";

fn main() {
    // Load the credentials to give to the auth service.
    dotenv::from_filename("credentials.env").ok().unwrap();

    let auth_json = json!({
        "data": { "user": "$USER", "password": "$PASSWORD" },
        "type": 1,
        "version": 5
    });

    let auth_string = auth_json.to_string();
    let auth_string = auth_string.replace("$USER", &env::var("user").unwrap());
    let auth_string = auth_string.replace("$PASSWORD", &env::var("password").unwrap());

    dbg!(&auth_string);
}
