use libconsensus_lachesis_rs::tcp_server::{TcpApp, TcpNode, TcpPeer};
use libconsensus_lachesis_rs::{BTreeHashgraph, Node, Swirlds};
use serde_derive::Deserialize;
use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use toml;
use vm::instruction::Program;
use vm::Cpu;

const DEFAULT_CPU_MEMORY: usize = 1024;
const DEFAULT_LACHESIS_PORT: usize = 9000;
const DEFAULT_SERVER_PORT: usize = 8080;

/// The initial configuration stored in `config.toml`.
#[derive(Debug, Deserialize)]
struct Config {
    cpu_memory: Option<usize>,
    lachesis_port: Option<usize>,
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

struct Server {
    node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>,
    port: usize,
}

impl Server {
    fn new(port: usize, node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>) -> Server {
        Server { node, port }
    }

    fn run(self, cpu_memory: usize) -> (JoinHandle<()>, JoinHandle<()>) {
        let server = self.get_server_handle();
        let node = self.node.clone();
        let queue_consumer = spawn(move || {
            let mut cpu = Cpu::new(cpu_memory).unwrap();
            loop {
                let events = node
                    .node
                    .get_ordered_events()
                    .expect("cannot get ordered events");
                let transactions: Vec<Vec<u8>> = events
                    .iter()
                    .flat_map(libconsensus_lachesis_rs::Event::transactions)
                    .collect();
                for transaction in transactions {
                    let program = Program::try_from(transaction.clone())
                        .expect("cannot convert transaction to program");
                    cpu.execute(program).expect("cannot execute program");
                }
                sleep(Duration::from_millis(100));
            }
        });
        (server, queue_consumer)
    }

    fn get_server_handle(&self) -> JoinHandle<()> {
        let port = self.port;
        let node = self.node.clone();
        spawn(move || {
            let address = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(address).unwrap();
            for stream_result in listener.incoming() {
                let mut stream = stream_result.unwrap();
                let mut content = Vec::new();
                stream.read_to_end(&mut content).unwrap();
                node.node.add_transaction(content).unwrap();
            }
        })
    }
}

/// CLI environment obtained by interpreting the initial configuration.
struct Env {
    // peers: Vec<TcpPeer>,
    // node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>,
    app: TcpApp,
    server: Server,
    cpu_memory: usize,
}

impl Env {
    pub fn new(config: Config) -> Self {
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
            "0.0.0.0:{}",
            config.lachesis_port.unwrap_or(DEFAULT_LACHESIS_PORT)
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
            // peers,
            // node,
            app,
            server,
            cpu_memory,
        }
    }
}

fn main() {
    env_logger::init();
    let config_raw = fs::read_to_string("config.toml").expect("cannot read config.toml");
    let env = Env::new(toml::from_str(config_raw.as_str()).expect("cannot parse config.toml"));
    let mut handles = Vec::new();
    let (handle1, handle2) = env.app.run().expect("app failed");
    handles.push(handle1);
    handles.push(handle2);
    let (server_handle1, server_handle2) = env.server.run(env.cpu_memory);
    handles.push(server_handle1);
    handles.push(server_handle2);
    for handle in handles {
        handle.join().expect("thread panicked")
    }
}
