use crate::citizen::main;

#[tokio::test]
async fn main_test() {
    main().await;
    assert!(true);
}