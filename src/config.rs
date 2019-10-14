use serde_derive::Deserialize;

use crate::constants::*;
use crate::dvm::DVM;
use failure::Error;
use failure::_core::fmt::Display;
use fvm::vm::VM;
use libcommon_rs::data::DataType;
use libcommon_rs::peer::Peer;
use libconsensus::{Consensus, ConsensusConfiguration};
use libconsensus_dag::{DAGPeer, DAGPeerList, DAGconfig, DAG};
use libhash_sha3::Hash;
use libsignature_ed25519_dalek::{PublicKey, SecretKey, Signature};
use libvm::DistributedVM;

/// The initial configuration stored in `config.toml`.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    cwd: String,
    serve_config: ServeConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ServeConfig {
    peers: Vec<PeerConfig>,
}

/// The structure of a peer record in the config file.
#[derive(Debug, Deserialize)]
pub(crate) struct PeerConfig {
    id: String,
    port: usize,
}

pub type EnvDAG = DAG<String, DAGData, SecretKey, PublicKey, Signature<Hash>>;

/// CLI environment obtained by interpreting the initial configuration.
// FIXME: Requires clarification of what it is supposed to accomplish.
pub(crate) struct Env {
    pub(crate) consensuses: Vec<EnvDAG>,
}
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DAGData(Vec<u8>);

impl Display for DAGData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Env {
    pub(crate) fn new(config: Config) -> Result<Self, Error> {
        let mut peers = vec![];
        for peer_config in config.serve_config.peers.iter() {
            let net_addr = format!("localhost:{}", peer_config.port);
            let peer: DAGPeer<_, PublicKey> = DAGPeer::new(peer_config.id.clone(), net_addr);
            peers.push(peer)
        }
        let peer_list = DAGPeerList::new_with_content(peers);
        let mut consensuses = vec![];
        for peer in config.serve_config.peers.iter() {
            let mut config: DAGconfig<String, DAGData, SecretKey, PublicKey> = DAGconfig::new();
            config.peers = peer_list.clone();
            config.request_addr = format!("localhost:{}", peer.port);
            config.reply_addr = format!("localhost:{}", peer.port + 1);
            consensuses.push(DAG::new(config)?);
        }
        Ok(Env { consensuses })
    }
}
