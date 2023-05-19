use dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tungstenite::connect;
use url::Url;

const AUTH_ADDRESS: &str = "wss://wirepas-dev.peercode.nl:8813";
const REALTIME_ADDRESS: &str = "wss://wirepas-dev.peercode.nl:8811";

fn main() {
    // Load the credentials to give to the auth service.
    dotenv::from_filename("credentials.env").ok().unwrap();

    let auth_json = json!({
        "data": { "username": "$USERNAME", "password": "$PASSWORD" },
        "type": 1,
        "version": 5
    });

    let auth_string = auth_json.to_string();
    let auth_string = auth_string.replace("$PASSWORD", &env::var("password").unwrap());
    let auth_string = auth_string.replace("$USERNAME", &env::var("username").unwrap());

    dbg!(&auth_string);

    let (mut socket, _) = connect(Url::parse(AUTH_ADDRESS).unwrap()).unwrap();
    socket
        .write_message(tungstenite::Message::Text(auth_string))
        .unwrap();
    let msg = socket.read_message().expect("problem reading msg");
    let as_str: &str = msg.to_text().unwrap();

    let obj: AuthResponse = serde_json::from_str(as_str).unwrap();

    dbg!(msg);
    dbg!(obj);
}

#[derive(Deserialize, Serialize, Debug)]
struct AuthData {
    role: usize,
    session_id: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct AuthResponse {
    data: AuthData,
    result: usize,
    r#type: usize,
    version: usize,
}
