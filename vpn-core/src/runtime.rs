
#[cfg(target_os = "linux")]
pub fn start_runtime<F: std::future::Future>(future: F) {
    tokio_uring::start(async move {
        future.await;
    });
}

#[cfg(not(target_os = "linux"))]
pub fn start_runtime<F: std::future::Future>(future: F) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);
}
