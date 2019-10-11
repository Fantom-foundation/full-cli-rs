use serde_derive::Deserialize;

use crate::constants::*;

/// The initial configuration stored in `config.toml`.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    cwd: String,
    serve_config: Option<ServeConfig>,
    generate_config: Option<GenerateConfig>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GenerateConfig {
    addresses: Vec<String>,
    n: u16,
    port_start: u16,
    port_increment: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ServeConfig {
    cpu_memory: Option<usize>,
    node_port: Option<usize>,
    server_port: Option<usize>,
    peers: Vec<PeerConfig>,
}

#[derive(Deserialize)]
pub(crate) struct ServeArgs {
    flag_config: Option<String>,
    flag_server_port: Option<usize>,
    flag_node_port: Option<usize>,
}

/// The structure of a peer record in the config file.
#[derive(Debug, Deserialize)]
pub(crate) struct PeerConfig {
    id: String,
    ip: String,
    port: usize,
}

/// CLI environment obtained by interpreting the initial configuration.
// FIXME: Requires clarification of what it is supposed to accomplish.
pub(crate) struct Env {
    server: Server,
    cpu_memory: usize,
}

impl Env {
    pub(crate) fn new(config: Config) -> Self {
        let peers: Vec<TcpPeer> = config
            .peers
            .iter()
            .map(|PeerConfig { id, ip, port }| TcpPeer {
                address: format!("{}:{}", ip, port),
                id: id.as_bytes().to_vec(),
            })
            .collect();
        let mut rng = ring::rand::SystemRandom::new();
        let local_address = format!(
            "{}:{}",
            LOCALHOST,
            config.node_port.unwrap_or(DEFAULT_NODE_PORT)
        );
        let cpu_memory = config.cpu_memory.unwrap_or(DEFAULT_CPU_MEMORY);
        Env {
            app,
            server,
            cpu_memory,
        }
    }
}
