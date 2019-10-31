//#[macro_use]
//extern crate serde;
//use std::fs;

use crate::config::PeerConfig;
use crate::config::ServeConfig;
//use docopt::Docopt;
use log::{debug, info};
//use toml;

mod config;
mod constants;
mod dvm;

use crate::config::{Config, Env};
use crate::dvm::DVM;
use evm_rs::transaction::Transaction;
use evm_rs::vm::VM;
use libvm::DistributedVM;

const VERSION: &str = env!("CARGO_PKG_VERSION");
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

/// Parses the command line arguments.
//fn parse_args() -> Result<Config, docopt::Error> {
//    Docopt::new(USAGE)?
//        .version(Some(VERSION.to_string()))
//        .parse()?
//        .deserialize()
//}

fn main() {
    env_logger::init();
    info!("DAG consensus CLI version {}", VERSION);
    //    let config = parse_args().unwrap_or_else(|e| e.exit());

    let config = Config {
        cwd: "./".to_string(),
        serve_config: ServeConfig {
            peers: vec![
                PeerConfig {
                    id: "первый".to_string(),
                    port: 9001,
                },
                PeerConfig {
                    id: "второй".to_string(),
                    port: 9003,
                },
                PeerConfig {
                    id: "третий".to_string(),
                    port: 9005,
                },
                PeerConfig {
                    id: "четвёртый".to_string(),
                    port: 9007,
                },
                PeerConfig {
                    id: "поехали".to_string(),
                    port: 9009,
                },
            ],
        },
    };
    debug!("Config: {:?}", config);

    let config_env = Env::new(config).unwrap();

    let mut threads = vec![];
    for c in config_env.consensuses {
        let t = std::thread::spawn(move || {
            let mut vm = DVM::default();
            let transaction: Transaction = Transaction {
                nonce: 0.into(),
                gas_price: 0.into(),
                start_gas: 0.into(),
                to: None,
                value: 0.into(),
                data: vec![],
                v: 0.into(),
                r: 0.into(),
                s: 0.into(),
            };
            vm.set_cpu(VM::new(vec![]));
            vm.set_consensus(c);
            vm.send_transaction(transaction).unwrap();
            vm.serve();
        });
        threads.push(t);
    }

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
