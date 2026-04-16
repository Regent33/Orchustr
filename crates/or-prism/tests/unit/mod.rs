use or_prism::{PrismError, install_global_subscriber};

#[test]
fn install_global_subscriber_accepts_valid_http_endpoint() {
    let result = install_global_subscriber("http://127.0.0.1:4318/v1/traces");
    assert!(
        result.is_ok(),
        "expected subscriber install to succeed: {result:?}"
    );
}

#[test]
fn install_global_subscriber_rejects_blank_endpoint() {
    let result = install_global_subscriber("   ");
    assert_eq!(
        result,
        Err(PrismError::InvalidEndpoint(
            "endpoint must not be blank".to_owned()
        ))
    );
}
