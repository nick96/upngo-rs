mod shared;

use shared::{get_client, init_logger, start_of_month};

#[test]
fn test_transactions_list() {
    init_logger();
    let transactions = get_client().transaction.list().exec().unwrap();
    assert!(transactions.is_ok());
}

#[test]
fn test_transaction_get() {
    init_logger();
    let id = std::env::var("UPBANK_TRANSACTION_ID").unwrap();
    let transaction = get_client().transaction.get(id).unwrap();
    assert!(transaction.is_ok());
}

#[test]
fn test_transaction_list_pagination() {
    init_logger();
    let transactions = get_client().transaction.list().size(1).exec().unwrap();
    assert!(transactions.is_ok());
    use upbank::response::Response;
    match transactions {
        Response::Ok(ts) => assert_eq!(ts.data.len(), 1),
        Response::Err(_) => unreachable!(),
    }
}

#[test]
fn test_transaction_list_filter_since() {
    init_logger();
    let start_of_month = start_of_month();
    let transactions = get_client()
        .transaction
        .list()
        .since(start_of_month)
        .exec()
        .unwrap();
    assert!(transactions.is_ok());
    use upbank::response::Response;
    let ts = match transactions {
        Response::Ok(ts) => ts,
        Response::Err(_) => unreachable!(),
    };
    assert!(!ts.data.is_empty());
    for t in ts.data {
        assert!(t.attributes.created_at >= start_of_month);
    }
}

#[test]
fn test_transaction_list_filter_until() {
    init_logger();
    let start_of_month = start_of_month();
    let transactions = get_client()
        .transaction
        .list()
        .until(start_of_month)
        .exec()
        .unwrap();
    assert!(transactions.is_ok());
    use upbank::response::Response;
    let ts = match transactions {
        Response::Ok(ts) => ts,
        Response::Err(_) => unreachable!(),
    };
    assert!(!ts.data.is_empty());
    for t in ts.data {
        assert!(t.attributes.created_at < start_of_month);
    }
}
