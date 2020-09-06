mod shared;

use shared::{get_client, init_logger};

#[test]
fn test_list_accounts() {
    init_logger();

    let accounts = get_client()
        .account
        .list()
        .exec()
        .expect("Error response from accounts request");
    assert!(
        accounts.is_ok(),
        "Expected accounts to be ok: {:?}",
        accounts
    );
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

#[test]
fn test_list_account_transactions() {
    init_logger();
    let account_id =
        std::env::var("UPBANK_ACCOUNT_ID").expect("No env var UPBANK_ACCOUNT_ID found");
    let transactions = get_client().account.transactions(account_id).unwrap();
    assert!(transactions.is_ok(), "Expected ok: {:?}", transactions);
}
