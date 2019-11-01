use crate::config::{DAGData, EnvDAG};
use ethereum_types::H160;
use evm_rs::transaction::Transaction;
use evm_rs::vm::{Opcode, VM};
use failure::Error;
use futures::executor::block_on;
use futures::stream::StreamExt;
use libconsensus::Consensus;
use libvm::DistributedVM;
use libconsensus_dag::DAGPeer;

pub struct DVM {
    cpu: Option<VM>,
    algorithm: Option<EnvDAG>,
}

impl Default for DVM {
    fn default() -> DVM {
        DVM {
            cpu: None,
            algorithm: None,
        }
    }
}

impl DVM {
    pub fn send_transaction(&mut self, transaction: Transaction) -> Result<(), Error> {
        if let Some(a) = &mut self.algorithm {
            a.send_transaction(transaction)?;
        }
        Ok(())
    }
}

impl<'a> DistributedVM<'a, VM, Opcode, DAGData, EnvDAG, H160> for DVM {
    fn set_cpu(&mut self, cpu: VM) {
        self.cpu = Some(cpu);
    }

    fn set_consensus(&mut self, algorithm: EnvDAG) {
        self.algorithm = Some(algorithm);
    }

    fn serve(mut self) {
        if let Some(a) = &mut self.algorithm {
            loop {
                // FIXME: check for exit condition here and do exit when met
                block_on(async {
                    if let Some(tx) = a.next().await {
                        // FIXME: we have received transaction tx from Consensus
                        // now we need to execute it on VM
                        println!("Got transaction: {}", tx);
                    }
                });
            }
        }
    }
}
