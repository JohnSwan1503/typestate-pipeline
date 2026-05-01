use super::{Author, Client, TestError};

pub async fn main() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-d", 1);

    // parallelism = 0 fails inside validate_and_finalize; the error
    // must bubble out through the chained pending future.
    let result = pipeline
        .tag_version(1)
        .with_parallelism(0)
        .validate_and_finalize()
        .deploy()
        .await;

    match result {
        Err(TestError::Invalid(msg)) => assert_eq!(msg, "parallelism must be > 0"),
        Ok(_) => panic!("expected validation error"),
    }
}
