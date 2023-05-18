use dotenv;
use serde_json::json;
use tungstenite::connect;
use url::Url;
use std::env;

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
    socket.write_message(tungstenite::Message::Text(auth_string)).unwrap();

    loop {
        let msg = socket.read_message().expect("problem reading msg");
        dbg!(msg);
    }
}
