mod shared;

use shared::*;
use log::*;

#[test]
fn test_list_accounts() {
    init_logger();

    let accounts = get_client()
        .account
        .list()
        .expect("Error response from accounts request");
    assert!(
        accounts.is_ok(),
        "Expected accounts to be ok: {:?}",
        accounts
    );
    info!("{:?}", accounts);
}

#[test]
fn test_get_account() {
    init_logger();
    let account_id =
        std::env::var("UPBANK_ACCOUNT_ID").expect("No env var UPBANK_ACCOUNT_ID found");
    let account = get_client()
        .account
        .get(account_id)
        .expect("Error response from account request");
    assert!(account.is_ok(), "Expected account to be ok: {:?}", account);
}
