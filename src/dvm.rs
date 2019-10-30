use crate::config::{DAGData, EnvDAG};
use ethereum_types::H160;
use evm_rs::transaction::Transaction;
use evm_rs::vm::{Opcode, VM};
use failure::Error;
use libconsensus::Consensus;
use libconsensus_dag::DAG;
use libvm::DistributedVM;

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
            let transaction_data = serde_json::to_vec(&transaction)?;
            let data = DAGData(transaction_data);
            a.send_transaction(data)?;
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

    fn serve(self) {
        unimplemented!()
    }
}
