// Tests for the util client.
mod shared;

use shared::{get_bad_client, get_client, init_logger};
use upbank::util::PingResponse;

#[test]
fn test_ping_ok() {
    init_logger();
    let ping = get_client().util.ping().expect("Request failed");
    if let PingResponse::Err(_) = ping {
        panic!("Expected response to be ok: {:?}", ping)
    }
}

#[test]
fn test_ping_401() {
    init_logger();
    let ping = get_bad_client().util.ping().expect("Request failed");
    let err_resp = match ping {
        PingResponse::Err(e) => e,
        _ => panic!("Expected response to be an error: {:?}", ping),
    };
    assert_eq!(err_resp.errors.len(), 1);
    let error = &err_resp.errors[0];
    assert_eq!(error.status, "401");
    assert_eq!(error.title, "Not Authorized");
    assert_eq!(error.detail, "The request was not authenticated because no valid credential was found in the Authorization header, or the Authorization header was not present.");
}
