use ethereum_types::H160;
use log::{debug, info};
use structopt::StructOpt;

use evm_rs::vm::VM;
use libvm::DistributedVM;

mod config;
mod constants;
mod dvm;

use crate::config::ServeConfig;
use crate::config::{Config, Env};
use crate::config::{EnvDAG, PeerConfig};
use crate::constants::DEFAULT_NODE_PORT;
use crate::dvm::DVM;
use std::thread::JoinHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, StructOpt)]
#[structopt(about = "Fully-participating node")]
enum Opt {
    Tester {
        #[structopt(short)]
        /// Number of nodes to test with
        n: usize,
    },
}

//const USAGE: &str = "
//DAG consensus CLI
//
//Usage:
//  full-cli-rs [--cwd <path>] serve [--config --server-port=<port> --node-port=<port>]
//  full-cli-rs [--cwd <path>] generate [--config --peers=<N> --all]
//  full-cli-rs [--cwd <path>] metric-parser <path>
//  full-cli-rs (--help | -h)
//  full-cli-rs --version
//
//Options:
//  -h, --help                  Show this message.
//  -v, --version               Show the version of the CLI.
//  -c, --config <file>         The configuration file path.
//  --cwd                       Change the current working directory. Defaults to argv[0].
//  -p, --server-port <port>    The server port.
//  -n, --node-port <port>      The consensus node port.
//  -a, --address <address>...  IP address. `--address 12.7.5 --address 12334.35.35.353`
//  --number-of-address <n>
//  --port-start <port>         9000 by default.
//  --port-increment <incr>     The increment. Defaults to 2. Must be > 1.
//";

fn main() {
    env_logger::init();
    info!("DAG consensus CLI version {}", VERSION);
    //    let config = parse_args().unwrap_or_else(|e| e.exit());
    let opt = Opt::from_args();
    let peers: Vec<PeerConfig> = match opt {
        Opt::Tester { n } => (0..n)
            .map(|i| PeerConfig {
                id: H160::random(),
                port: DEFAULT_NODE_PORT + i * 2,
            })
            .collect(),
        // _ => unimplemented!(),
    };

    let config = Config {
        cwd: "./".to_string(),
        serve_config: ServeConfig { peers },
    };
    debug!("Config: {:?}", config);

    let config_env = Env::new(config).unwrap();

    let mut vms = Vec::new();
    let mut vm_senders = Vec::new();

    for c in config_env.consensuses {
        let (mut vm, tx) = DVM::new();
        vm.set_cpu(VM::new(vec![]));
        vm.set_consensus(c);
        vms.push(vm);
        vm_senders.push(tx);
    }

    let threads: Vec<JoinHandle<()>> = vms
        .into_iter()
        .map(|v| {
            std::thread::spawn(move || {
                v.serve();
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    /*

    let peer_list = instantiate peer list (initial peers)?;
    let transport = instantiate transport<UDP>(host, port)?;
    let communication = instantiate communication<Channels>()?;
    let vm = instantiate_vm(&communication)?;
    let consensus = instantiate consensus(&peer_list, &transport, &communication, &vm)?;

    start(consensus)?;
    */

    /*
      let vm = VM<SolidityVM<DAGConsensus<Transport<TCP>,


    let vm = VM::Solidity<Consensus::DAG<Transport::TCP>,

     vm.run()


    let peer_list = instantiate peer list (initial peers)?;
    let transport = instantiate transport<UDP>(host, port)?;
    let communication = instantiate communication<Channels>()?;
    let consensus = instantiate consensus(&communication)?;
    let vm = instantiate_vm(&peer_list, &transport, &communication, &consensus)?;
    // alternatively provide a big configuration object, and pass that to `instantiate_vm`

    start(consensus)?;


    // alternative
    start_vm(**config)
    */
}
