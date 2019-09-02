mod constants;
mod executor;
mod server;

use std::fs;
use std::sync::Arc;

use docopt::Docopt;
use failure;
use futures::{self, executor::block_on};
use libconsensus_lachesis_rs::tcp_server::{TcpApp, TcpNode, TcpPeer};
use log::{debug, error, info};
use serde_derive::Deserialize;
use toml;

use crate::constants::*;
use crate::server::Server;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE: &str = "
DAG consensus CLI

Usage:
  full-cli-rs [options]
  full-cli-rs (--help | -h)
  full-cli-rs --version

Options:
  -h, --help                Show this message.
  -v, --version             Show the version of the CLI.
  -c, --config <file>       The configuration file path.
  -p, --server-port <port>  The server port.
  -n, --node-port <port>    The consensus node port.
";

#[derive(Deserialize)]
struct Args {
    flag_config: Option<String>,
    flag_server_port: Option<usize>,
    flag_node_port: Option<usize>,
}

/// The initial configuration stored in `config.toml`.
#[derive(Debug, Deserialize)]
struct Config {
    cpu_memory: Option<usize>,
    node_port: Option<usize>,
    server_port: Option<usize>,
    peers: Vec<PeerConfig>,
}

/// The structure of a peer record in the config file.
#[derive(Debug, Deserialize)]
struct PeerConfig {
    id: String,
    ip: String,
    port: usize,
}

/// CLI environment obtained by interpreting the initial configuration.
// FIXME: Requires clarification of what it is supposed to accomplish.
struct Env {
    app: TcpApp,
    server: Server,
    cpu_memory: usize,
}

impl Env {
    fn new(config: Config) -> Self {
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
        let node = Arc::new(TcpNode::new(&mut rng, local_address).unwrap());
        for peer in peers.iter() {
            node.node.add_node(Arc::new(peer.clone())).unwrap();
        }
        let app = TcpApp::new(node.clone());
        let server = Server::new(
            config.server_port.unwrap_or(DEFAULT_SERVER_PORT),
            node.clone(),
        );
        let cpu_memory = config.cpu_memory.unwrap_or(DEFAULT_CPU_MEMORY);
        Env {
            app,
            server,
            cpu_memory,
        }
    }

    fn execute(&self) {
        // TODO: to be removed after defining a `Future` for `TcpApp`.
        let mut app_threads = Vec::new();
        // TODO: define an instance of `Future` for `TcpApp` and join the resulting futures with the
        // ones of the `Server` in `Env::execute`.
        let (thread1, thread2) = self.app.clone().run().expect("app failed");
        app_threads.push(thread1);
        app_threads.push(thread2);
        match block_on(self.server.run(self.cpu_memory)) {
            (Ok(()), _) => {}
            (Err(e), _) => error!("{}", e),
        }
        // TODO: decommission as soon as `TcpApp` implements `Future`.
        for thread in app_threads.drain(..) {
            thread.join().expect("thread panicked");
        }
    }
}

/// Parses the command line arguments.
fn parse_args() -> Result<Args, docopt::Error> {
    Docopt::new(USAGE)?
        .version(Some(VERSION.to_string()))
        .parse()?
        .deserialize()
}

fn main() {
    env_logger::init();
    info!("DAG consensus CLI version {}", VERSION);
    let args = parse_args().unwrap_or_else(|e| e.exit());
    let config_raw = fs::read_to_string(
        args.flag_config
            .unwrap_or_else(|| String::from("config.toml")),
    )
    .expect("cannot read config.toml");
    let mut config: Config = toml::from_str(config_raw.as_str()).expect("cannot parse config.toml");
    if let Some(server_port) = args.flag_server_port {
        config.server_port = Some(server_port);
    }
    if let Some(node_port) = args.flag_node_port {
        config.node_port = Some(node_port);
    }
    debug!("Config: {:?}", config);
    Env::new(config).execute();
}
