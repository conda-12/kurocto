use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread,
    time::Duration,
};

use nix::unistd::getuid;
use release_channel::ReleaseChannel;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const CONNECT_TIMEOUT: Duration = Duration::from_millis(10);
const RECEIVE_TIMEOUT: Duration = Duration::from_millis(35);
const SEND_TIMEOUT: Duration = Duration::from_millis(20);
const USER_BLOCK: u16 = 100;

// Deliberately different from the base upstream Zed uses (43737), so that both
// editors can run side by side for the same user without sharing a port.
const PORT_BASE: u16 = 47737;

fn channel_port(release_channel: ReleaseChannel) -> u16 {
    match release_channel {
        ReleaseChannel::Dev => PORT_BASE,
        ReleaseChannel::Preview => PORT_BASE + USER_BLOCK,
        ReleaseChannel::Stable => PORT_BASE + (2 * USER_BLOCK),
        ReleaseChannel::Nightly => PORT_BASE + (3 * USER_BLOCK),
    }
}

fn address() -> SocketAddr {
    // These port numbers are offset by the user ID to avoid conflicts between
    // different users on the same machine. In addition to that the ports for each
    // release channel are spaced out by 100 to avoid conflicts between different
    // users running different release channels on the same machine. This ends up
    // interleaving the ports between different users and different release channels.
    //
    // On macOS user IDs start at 501 and on Linux they start at 1000. The first user
    // on a Mac with ID 501 running a dev channel build will use port 48238, and the
    // second user with ID 502 will use port 48239, and so on. User 501 will use ports
    // 48338, 48438, and 48538 for the preview, stable, and nightly channels,
    // respectively. User 502 will use ports 48339, 48439, and 48539 for the preview,
    // stable, and nightly channels, respectively.
    let port = channel_port(*release_channel::RELEASE_CHANNEL);
    let uid = getuid().as_raw();
    // Ensure that the user ID is not too large to avoid overflow when
    // calculating the port number. This seems unlikely but it doesn't
    // hurt to be safe.
    let max_uid = u32::from(u16::MAX) - u32::from(port);
    let wrapped_uid = (uid % max_uid) as u16;
    let user_port = port + wrapped_uid;

    SocketAddr::V4(SocketAddrV4::new(LOCALHOST, user_port))
}

fn channel_handshake(release_channel: ReleaseChannel) -> &'static str {
    match release_channel {
        ReleaseChannel::Dev => "Kurocto Editor Dev Instance Running",
        ReleaseChannel::Nightly => "Kurocto Editor Nightly Instance Running",
        ReleaseChannel::Preview => "Kurocto Editor Preview Instance Running",
        ReleaseChannel::Stable => "Kurocto Editor Stable Instance Running",
    }
}

fn instance_handshake() -> &'static str {
    channel_handshake(*release_channel::RELEASE_CHANNEL)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsOnlyInstance {
    Yes,
    No,
}

pub fn ensure_only_instance() -> IsOnlyInstance {
    if check_got_handshake() {
        return IsOnlyInstance::No;
    }

    let listener = match TcpListener::bind(address()) {
        Ok(listener) => listener,

        Err(err) => {
            log::warn!("Error binding to single instance port: {err}");
            if check_got_handshake() {
                return IsOnlyInstance::No;
            }

            // Avoid failing to start when some other application by chance already has
            // a claim on the port. This is sub-par as any other instance that gets launched
            // will be unable to communicate with this instance and will duplicate
            log::warn!("Backup handshake request failed, continuing without handshake");
            return IsOnlyInstance::Yes;
        }
    };

    thread::Builder::new()
        .name("EnsureSingleton".to_string())
        .spawn(move || {
            for stream in listener.incoming() {
                let mut stream = match stream {
                    Ok(stream) => stream,
                    Err(_) => return,
                };

                _ = stream.set_nodelay(true);
                _ = stream.set_read_timeout(Some(SEND_TIMEOUT));
                _ = stream.write_all(instance_handshake().as_bytes());
            }
        })
        .unwrap();

    IsOnlyInstance::Yes
}

fn check_got_handshake() -> bool {
    match TcpStream::connect_timeout(&address(), CONNECT_TIMEOUT) {
        Ok(mut stream) => {
            let mut buf = vec![0u8; instance_handshake().len()];

            stream.set_read_timeout(Some(RECEIVE_TIMEOUT)).unwrap();
            if let Err(err) = stream.read_exact(&mut buf) {
                log::warn!("Connected to single instance port but failed to read: {err}");
                return false;
            }

            if buf == instance_handshake().as_bytes() {
                log::info!("Got instance handshake");
                return true;
            }

            log::warn!("Got wrong instance handshake value");
            false
        }

        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_identifies_kurocto() {
        for release_channel in ReleaseChannel::ALL {
            let handshake = channel_handshake(release_channel);
            assert!(handshake.starts_with("Kurocto Editor "), "{handshake}");
            assert!(handshake.ends_with(" Instance Running"), "{handshake}");
            assert!(!handshake.contains("Zed"), "{handshake}");
        }
    }

    #[test]
    fn test_ports_do_not_overlap_upstream_zed() {
        const UPSTREAM_ZED_PORT_BASE: u16 = 43737;
        let upstream_ports: Vec<u16> = (0..4)
            .map(|index| UPSTREAM_ZED_PORT_BASE + index * USER_BLOCK)
            .collect();
        let mut ports = Vec::new();
        for release_channel in ReleaseChannel::ALL {
            let port = channel_port(release_channel);
            assert!(!upstream_ports.contains(&port), "{port}");
            assert!(!ports.contains(&port), "{port}");
            ports.push(port);
        }
    }
}
