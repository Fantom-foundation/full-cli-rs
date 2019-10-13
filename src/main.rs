use std::fs;

use docopt::Docopt;
use log::{debug, info};
use toml;

mod config;
mod constants;

use crate::config::{Config, Env, ServeArgs};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE: &str = "
DAG consensus CLI

Usage:
  full-cli-rs [--cwd <path>] serve [--config --server-port=<port> --node-port=<port>]
  full-cli-rs [--cwd <path>] generate [--config --peers=<N> --all]
  full-cli-rs [--cwd <path>] metric-parser <path>
  full-cli-rs (--help | -h)
  full-cli-rs --version

Options:
  -h, --help                  Show this message.
  -v, --version               Show the version of the CLI.
  -c, --config <file>         The configuration file path.
  --cwd                       Change the current working directory. Defaults to argv[0].
  -p, --server-port <port>    The server port.
  -n, --node-port <port>      The consensus node port.
  -a, --address <address>...  IP address. `--address 12.7.5 --address 12334.35.35.353`
  --number-of-address <n>
  --port-start <port>         9000 by default.
  --port-increment <incr>     The increment. Defaults to 2. Must be > 1.
";

/// Parses the command line arguments.
fn parse_args() -> Result<ServeArgs, docopt::Error> {
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
    let config_env = Env::new(config);

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
