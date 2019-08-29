//! # FANTOM full cli rs
//!
//! Fully participating node—full-CLI—interface to the Fantom blockchain.
//!
//! ## FEATURES:
//!
//! Handles peer based consensus via a set of configurable protocols.
//! Current protocols supported: TCP
//! Protocols to be added: UDP, Unix Sockets
//!
//! ### Example Command:
//!
//! RUST_LOG=debug cargo run -- -c config/config.toml -s 10001 -n 11001 -p tcp
//! RUST_LOG=debug cargo run -- -c config/config.toml -s 80 -n 81 -p udp
//!
//! See USAGE variable below to see input options below.

#![feature(async_await)]
mod constants;
mod executor;
mod server;

use std::fs;
use std::sync::Arc;

use docopt::Docopt;
use failure;
use futures::{self, executor::block_on, StreamExt};
use libtransport::{Transport, TransportConfiguration};
use log::{debug, error, info};
use serde_derive::Deserialize;
use toml;

use crate::constants::*;
use libtransport::errors::Error;
use libtransport::generic_test::{Data, Id, TestPeerList};
use libtransport_tcp::{TCPtransport, TCPtransportCfg};
use libconsensus::TransactionType;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Docopt usage string configuration, currently takes in 6 variables:
/// -h: help
/// -v: version
/// -c: config file
/// -s: server port
/// -n: node port
/// -p: transport string

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
  -s, --server-port <port>  The server port.
  -n, --node-port <port>    The consensus node port.
  -p, --protocol <String>   The transport protocol.
";

/// Command line argument flags.
///
/// Searches the docopt cli args for specific flags, these include:
/// > Config file location (String)
/// > Server port (usize)
/// > Node Port (usize)
/// > Transport Protocol (String)

/// Used in conjunction with docopt to parse cli arguments.
#[derive(Deserialize, Debug)]
struct Args {
    flag_config: Option<String>,
    flag_server_port: Option<usize>,
    flag_node_port: Option<usize>,
    flag_protocol: Option<String>,
}

/// The initial configuration stored in `config.toml`.
/// Passed into Env for starting node peer network.
#[derive(Debug, Deserialize)]
struct Config {
    cpu_memory: Option<usize>,
    node_port: Option<usize>,
    server_port: Option<usize>,
    peers: Vec<PeerConfig>,
    protocol: Option<String>,
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
    //   server: Server,
    cpu_memory: usize,
}

impl Env {

    /// Creates new instance of Env struct. Requires a 'Config' struct to configure peers and establish
    /// node network.
    fn new(config: Config) -> Self {

        // Create a Vector of strings containing Ip + Port as TCP socket details.
        let peers: Vec<String> = config
            .peers
            .iter()
            .map(|PeerConfig { id, ip, port }| format!("{}:{}", ip, port))
            .collect();

        // Set CPU memory for VM.
        let cpu_memory = config.cpu_memory.unwrap_or(DEFAULT_CPU_MEMORY);

        // Randomiser
        let mut rng = ring::rand::SystemRandom::new();

        // Get the local address for each node specific to its port.
        let local_address = format!(
            "{}:{}",
            LOCALHOST,
            config.node_port.unwrap_or(DEFAULT_NODE_PORT)
        );

        // Check for the transport protocol and execute correct procedure for each input.
        if let Some(protocol) = config.protocol.as_ref() {
            match protocol.as_str() {
                // TODO: Define proper method to handle setting up a tcp-based set of nodes.
                "tcp" => { libtransport::generic_test::common_test::<TCPtransportCfg<Data>, TCPtransport<Data>>(peers);},
                _ => { panic!("No valid transport protocol has been specified!"); }
            }
        } else {
            panic!("No transport protocol specified!");
        }

        // Return Env
        Env { cpu_memory }
    }

    /// TODO: Create VMs for each peer, add consensus as argument, and start network.
    fn execute(&self) {
        //        // TODO: to be removed after defining a `Future` for `TcpApp`.
        //        let mut app_threads = Vec::new();
        //        // TODO: define an instance of `Future` for `TcpApp` and join the resulting futures with the
        //        // ones of the `Server` in `Env::execute`.
        //        let (thread1, thread2) = self.app.clone().run().expect("app failed");
        //        app_threads.push(thread1);
        //        app_threads.push(thread2);
        //        match block_on(self.server.run(self.cpu_memory)) {
        //            (Ok(()), _) => {}
        //            (Err(e), _) => error!("{}", e),
        //        }
        //        // TODO: decommission as soon as `TcpApp` implements `Future`.
        //        for thread in app_threads.drain(..) {
        //            thread.join().expect("thread panicked");
        //        }
    }
}

/// Parses the command line arguments. (Docopt)
fn parse_args() -> Result<Args, docopt::Error> {
    Docopt::new(USAGE)?
        .version(Some(VERSION.to_string()))
        .parse()?
        .deserialize()
}

fn main() {
    env_logger::init();

    info!("DAG consensus CLI version {}", VERSION);

    // Use Docopt to parse cli arguments into a set of variables to be used in future.
    let args = parse_args().unwrap_or_else(|e| e.exit());

    // Gather variables based on flags set in the Args struct.
    let config_raw = fs::read_to_string(
        args.flag_config
            .unwrap_or_else(|| String::from("config.toml")),
    )
    .expect("cannot read config.toml");

    // Populate config string with data from config.toml
    let mut config: Config = toml::from_str(config_raw.as_str()).expect("cannot parse config.toml");

    // Extract server port argument and add to config.
    if let Some(server_port) = args.flag_server_port {
        config.server_port = Some(server_port);
    }

    // Extract node port argument and add to config.
    if let Some(node_port) = args.flag_node_port {
        config.node_port = Some(node_port);
    }

    // Extract transport protocol argument and add to config.
    if let Some(protocol) = args.flag_protocol {
        config.protocol = Some(protocol);
    }

    // Debug purposes - remove when not needed.
    debug!("Config: {:?}", config);

    // Finally, create Env struct and add config to it.
    let env = Env::new(config);

    // TODO: Add function to execute data based on config and start the peer network.
    // env.execute();
}
