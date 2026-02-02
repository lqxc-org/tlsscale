use anyhow::Result;

pub mod wireguard;
pub mod dns;
pub mod proxy;

/// The abstract interface for a VPN Device implementation.
/// Uses native async fn (Return Position Impl Trait In Trait) available in Rust 2024.
#[allow(async_fn_in_trait)]
pub trait VpnDevice {
    /// Start the VPN device/interface
    async fn start(&self) -> Result<()>;
    
    /// Stop the VPN device
    async fn stop(&self) -> Result<()>;

    /// Send a packet through the VPN tunnel
    async fn send(&self, buf: &[u8]) -> Result<usize>;

    /// Receive a packet from the VPN tunnel
    async fn recv(&self, buf: &mut [u8]) -> Result<usize>;
}

pub fn init_tracing() {
    log::info!("Tracing initialized");
}