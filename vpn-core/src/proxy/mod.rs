pub struct ReverseProxy {
    pub bind_addr: String,
}

impl ReverseProxy {
    pub fn new(bind_addr: String) -> Self {
        Self { bind_addr }
    }

    pub async fn start(&self) {
        log::info!("Starting Reverse Proxy on {}", self.bind_addr);
    }
}
