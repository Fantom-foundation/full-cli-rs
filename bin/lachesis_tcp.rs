#[macro_use]
extern crate structopt;
extern crate libcli;
extern crate liblachesis_lachesis_rs;

use libcli::LogOptions;
use liblachesis_lachesis_rs::tcp_server::{TcpApp, TcpNode, TcpPeer};
use std::env::args;
use std::sync::Arc;
use structopt::StructOpt;

const BASE_PORT: usize = 9000;
const USAGE: &'static str = "Usage: tcp-client [number of nodes] [consensus-algorithm]";

// CLI options and switches
#[derive(Debug, StructOpt)]
/// Lachesis consensus node
struct Opt {
    #[structopt(long = "datadar", parse(from_os_str))]
    /// Top-level directory for configuration and data
    data_dir: Option<String>,

    #[structopt(long = "log")]
    /// Log level: debug, info, warn, error, fatal, panic
    log: Option<LogOptions>,

    #[structopt(long = "log2file", default_value = "false")]
    /// duplicate log output into file lachesis_<BindAddr>.log
    log2file: bool,

    #[structopt(long = "syslog", default_value = "false")]
    /// duplicate log output into syslog
    syslog: bool,

    #[structopt(long = "pidfile", parse(from_os_str), default_value = "/tmp/false")]
    /// pidfile location; /tmp/rs-lachesis.pid by default
    syslog: Option<String>,

    #[structopt(name = "listen", long, short)]
    /// Listen IP:Port for lachesis node
    listen: Option<String>,

    #[structopt(name = "timeout", long, short, default_value = "180")]
    /// TCP timeout in seconds
    timeout: i64,

    #[structopt(name = "proxy-listen", short, long)]
    /// Listen IP:Port for lachesis proxy
    proxy_listen: Option<String>,

    #[structopt(name = "heartbeat", long, default_value = "5")]
    /// Time between gossips
    heartbeat: i64,

    #[structopt(name = "peer-selector", long, default_value = "random")]
    peer_selector: PeerSelectorOptions,
}
/**
 * Main lachesis-rs TCP client entrypoint. Starts multiple TCP node peers.
 */
fn main() {
    let opt = Opt::from_args();
    env_logger::init();
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        panic!(USAGE);
    }
    let mut rng = ring::rand::SystemRandom::new();
    let n_nodes = args[1].parse::<usize>().unwrap();
    let algorithm = args[2].clone();
    let mut nodes = Vec::with_capacity(n_nodes);
    let mut peers = Vec::with_capacity(n_nodes);
    for i in 0..n_nodes {
        let a = format!("0.0.0.0:{}", BASE_PORT + i);
        let node = TcpNode::new(&mut rng, a.clone()).unwrap();
        peers.push(TcpPeer {
            address: a,
            id: node.node.get_id().clone(),
        });
        nodes.push(Arc::new(node));
    }
    for node in nodes.iter() {
        for peer in peers.iter() {
            if peer.id.clone() != node.node.get_id() {
                node.node.add_node(Arc::new(peer.clone())).unwrap();
            }
        }
    }
    let mut handles = Vec::with_capacity(n_nodes * 2);
    for node in nodes {
        let app = TcpApp::new(node.clone());
        let (handle1, handle2) = app.run().unwrap();
        handles.push(handle1);
        handles.push(handle2);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
