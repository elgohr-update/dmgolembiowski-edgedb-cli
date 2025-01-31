use std::io;
use std::time::Duration;

use async_std::future::Future;


pub async fn timeout<F, T, E>(dur: Duration, f: F) -> anyhow::Result<T>
    where F: Future<Output = Result<T, E>>,
          E: Into<anyhow::Error>,
{
    use async_std::future::timeout;

    timeout(dur, f).await
    .map(|r| r.map_err(Into::into))
    .unwrap_or_else(|_| Err(io::Error::from(io::ErrorKind::TimedOut).into()))
}
