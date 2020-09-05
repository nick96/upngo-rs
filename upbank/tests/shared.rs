use std::sync::Once;
use upbank::Client;

static START: Once = Once::new();

pub fn init_logger() {
    START.call_once(|| {
        pretty_env_logger::init();
    });
}

pub fn get_token() -> String {
    std::env::var("UPBANK_TOKEN")
        .expect("There's not environment variable UPBANK_TOKEN. It is require")
}

pub fn get_client() -> Client {
    Client::default_client(get_token())
}

pub fn get_bad_client() -> Client {
    Client::default_client("not-a-real-token".to_string())
}
