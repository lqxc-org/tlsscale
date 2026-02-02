pub struct DnsResolver {
    // Placeholder for upstream DNS servers
    pub upstreams: Vec<String>,
}

impl DnsResolver {
    pub fn new(upstreams: Vec<String>) -> Self {
        Self { upstreams }
    }

    pub async fn resolve(&self, domain: &str) {
        log::info!("Resolving {} using upstreams {:?}", domain, self.upstreams);
    }
}
