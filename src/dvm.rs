use crate::config::{DAGData, EnvDAG};
use ethereum_types::H160;
use evm_rs::vm::{Opcode, VM};
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
