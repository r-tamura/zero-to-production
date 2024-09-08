use tokio::task::JoinHandle;

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let curent_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || curent_span.in_scope(f))
}
