use anyhow::Result;
use std::net::SocketAddr;
use tokio_uring::net::UdpSocket;
use vpn_core::VpnDevice;

pub struct LinuxVpnDevice {
    socket: UdpSocket,
}

impl LinuxVpnDevice {
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Self { socket })
    }
}

impl VpnDevice for LinuxVpnDevice {
    async fn start(&self) -> Result<()> {
        log::info!("Linux VPN Device started (using tokio-uring)");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        log::info!("Linux VPN Device stopped");
        Ok(())
    }

    async fn send(&self, buf: &[u8]) -> Result<usize> {
        let owned = buf.to_vec();
        let (res, _) = self
            .socket
            .send_to(owned, "127.0.0.1:0".parse().unwrap())
            .await;
        res.map_err(|e| e.into())
    }

    async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        let temp = vec![0u8; buf.len()];
        let (res, filled) = self.socket.recv_from(temp).await;
        match res {
            Ok((len, _)) => {
                buf[..len].copy_from_slice(&filled[..len]);
                Ok(len)
            }
            Err(e) => Err(e.into()),
        }
    }
}

pub fn start_runtime<F: std::future::Future>(future: F) {
    tokio_uring::start(async move {
        future.await;
    });
}
