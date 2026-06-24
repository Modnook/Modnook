use std::future::Future;

pub trait Cache {
    type Error;

    fn get<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Option<T>, Self::Error>> + Send;

    fn set<T: serde::Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: i64,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
