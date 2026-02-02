use anyhow::Result;
use vpn_core::VpnDevice;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub struct WindowsVpnDevice {
    socket: UdpSocket,
}

impl WindowsVpnDevice {
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Self { socket })
    }
}

impl VpnDevice for WindowsVpnDevice {
    async fn start(&self) -> Result<()> {
        log::info!("Windows VPN Device started");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    async fn send(&self, buf: &[u8]) -> Result<usize> {
         self.socket.send_to(buf, "127.0.0.1:0".parse::<SocketAddr>().unwrap()).await.map_err(|e| e.into())
    }

    async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        let (len, _) = self.socket.recv_from(buf).await?;
        Ok(len)
    }
}

pub fn start_runtime<F: std::future::Future>(future: F) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);
}
