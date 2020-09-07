mod shared;

use shared::{init_logger, get_client};

#[test]
fn test_list_transactions() {
    init_logger();
    let transactions = get_client().transaction.list().exec().unwrap();
    assert!(transactions.is_ok());
}

#[test]
fn test_get_transaction() {
    init_logger();
    let id = std::env::var("UPBANK_TRANSACTION_ID").unwrap();
    let transaction = get_client().transaction.get(id).unwrap();
    assert!(transaction.is_ok());
}
