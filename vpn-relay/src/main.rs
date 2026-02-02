use anyhow::Result;
use clap::Parser;
use vpn_core::VpnDevice;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 51820)]
    port: u16,
}

#[cfg(target_os = "linux")]
use vpn_linux::{LinuxVpnDevice as PlatformDevice, start_runtime};

#[cfg(target_os = "macos")]
use vpn_macos::{MacOsVpnDevice as PlatformDevice, start_runtime};

#[cfg(target_os = "windows")]
use vpn_windows::{WindowsVpnDevice as PlatformDevice, start_runtime};

fn main() -> Result<()> {
    vpn_core::init_tracing();
    let args = Args::parse();

    start_runtime(async move {
        log::info!("Starting VPN Relay on port {}", args.port);

        let addr = format!("0.0.0.0:{}", args.port).parse().unwrap();
        let device = PlatformDevice::new(addr)
            .await
            .expect("Failed to create device");

        if let Err(e) = run(device).await {
            log::error!("Error: {}", e);
        }
    });

    Ok(())
}

async fn run(device: impl VpnDevice) -> Result<()> {
    device.start().await?;

    let mut buf = vec![0u8; 1500];
    loop {
        match device.recv(&mut buf).await {
            Ok(n) => {
                log::info!("Received {} bytes", n);
                // Echo back for testing
                let _ = device.send(&buf[..n]).await;
            }
            Err(e) => {
                log::error!("Recv error: {}", e);
            }
        }
    }
}
