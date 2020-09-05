// Tests for the util client.
use std::sync::Once;
use upbank::{Client, response::Response};

static START: Once = Once::new();

fn init_logger() {
    START.call_once(|| {
        pretty_env_logger::init();
    });
}

fn get_token() -> String {
    std::env::var("UPBANK_TOKEN")
        .expect("There's not environment variable UPBANK_TOKEN. It is require")
}

fn get_client() -> Client {
    Client::default_client(get_token())
}

fn get_bad_client() -> Client {
    Client::default_client("not-a-real-token".to_string())
}

#[test]
fn test_ping_ok() {
    init_logger();
    let ping = get_client().util.ping().expect("Request failed");
    assert!(ping.is_ok(), "Expected response to be ok: {:?}", ping);
}

#[test]
fn test_ping_401() {
    init_logger();
    let ping = get_bad_client().util.ping().expect("Request failed");
    assert!(ping.is_err(), "Expected response to be an error: {:?}", ping);
    let err_resp = match ping {
        Response::Err(e) => e,
        _ => unreachable!(),
    };
    assert_eq!(err_resp.errors.len(), 1);
    let error = &err_resp.errors[0];
    assert_eq!(error.status, "401");
    assert_eq!(error.title, "Not Authorized");
    assert_eq!(error.detail, "The request was not authenticated because no valid credential was found in the Authorization header, or the Authorization header was not present.");
}
