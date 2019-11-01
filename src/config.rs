use crate::constants::LOCALHOST;
use ethereum_types::H160;
use serde_derive::Deserialize;

//use crate::constants::*;
//use crate::dvm::DVM;
//use evm_rs::vm::VM;
use failure::Error;
//use libcommon_rs::data::DataType;
use libcommon_rs::peer::Peer;
use libconsensus::{Consensus, ConsensusConfiguration};
use libconsensus_dag::{DAGPeer, DAGPeerList, DAGconfig, DAG};
use libhash_sha3::Hash;
use libsignature_ed25519_dalek::{PublicKey, SecretKey, Signature};
//use libvm::DistributedVM;
use libcommon_rs::peer::PeerList;
use libsignature::Signature as LibSignature;

/// The initial configuration stored in `config.toml`.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) cwd: String,
    pub(crate) serve_config: ServeConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ServeConfig {
    pub(crate) peers: Vec<PeerConfig>,
}

/// The structure of a peer record in the config file.
#[derive(Debug, Deserialize)]
pub(crate) struct PeerConfig {
    pub(crate) id: H160,
    pub(crate) port: usize,
}

pub type DAGData = evm_rs::transaction::Transaction;

pub type EnvDAG = DAG<H160, DAGData, SecretKey, PublicKey, Signature<Hash>>;

/// CLI environment obtained by interpreting the initial configuration.
// FIXME: Requires clarification of what it is supposed to accomplish.
pub(crate) struct Env {
    pub(crate) consensuses: Vec<EnvDAG>,
}

impl Env {
    pub(crate) fn new(config: Config) -> Result<Self, Error> {
        let n = config.serve_config.peers.len();
        let mut kp: Vec<(PublicKey, SecretKey)> = Vec::with_capacity(n);
        let mut peer_list = DAGPeerList::<H160, PublicKey>::default();

        for i in 0..n {
            kp.push(Signature::<Hash>::generate_key_pair().unwrap());
            let net_addr = format!("{}:{}", LOCALHOST, config.serve_config.peers[i].port);
            let mut peer: DAGPeer<H160, PublicKey> =
                DAGPeer::new(config.serve_config.peers[i].id.clone(), net_addr);
            peer.set_public_key(kp[i].0.clone());
            peer_list.add(peer)?;
        }

        let mut consensuses = vec![];
        for i in 0..n {
            let mut cfg: DAGconfig<H160, DAGData, SecretKey, PublicKey> = DAGconfig::new();
            cfg.peers = peer_list.clone();
            cfg.request_addr = format!("{}:{}", LOCALHOST, config.serve_config.peers[i].port);
            cfg.reply_addr = format!("{}:{}", LOCALHOST, config.serve_config.peers[i].port + 1);
            cfg.transport_type = libtransport::TransportType::TCP;
            cfg.store_type = libcommon_rs::store::StoreType::Sled;
            cfg.creator = config.serve_config.peers[i].id.clone();
            cfg.public_key = kp[i].0.clone();
            cfg.secret_key = kp[i].1.clone();
            consensuses.push(DAG::new(cfg)?);
        }
        Ok(Env { consensuses })
    }
}
