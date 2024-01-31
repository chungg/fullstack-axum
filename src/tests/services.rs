use crate::core::services::random_data;

#[test]
fn test_random_endpoint() {
    let data = random_data();
    assert!(data.as_object().unwrap()["datasets"][0]["data"].is_array());
    assert_eq!(
        data.as_object().unwrap()["datasets"][0]["data"]
            .as_array()
            .unwrap()
            .len(),
        6
    );
}
