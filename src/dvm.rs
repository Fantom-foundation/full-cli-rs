use ethereum_types::H160;
use evm_rs::transaction::Transaction;
use evm_rs::vm::{Opcode, VM};
use futures::channel::mpsc;
use futures::executor::block_on;
use futures::pin_mut;
use futures::select;
use futures::stream::StreamExt;
use futures::FutureExt;
use libconsensus::Consensus;
use libvm::DistributedVM;

use crate::config::{DAGData, EnvDAG};

pub struct DVM {
    cpu: Option<VM>,
    algorithm: Option<EnvDAG>,
    rx: mpsc::UnboundedReceiver<Transaction>,
}

impl DVM {
    pub fn new() -> (DVM, mpsc::UnboundedSender<Transaction>) {
        let (tx, rx) = mpsc::unbounded();
        return (
            DVM {
                cpu: None,
                algorithm: None,
                rx,
            },
            tx,
        );
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
        if let (Some(consensus), Some(cpu)) = (&mut self.algorithm, &mut self.cpu) {
            loop {
                // FIXME: check for exit condition here and do exit when met
                let send_local = self.rx.next().fuse();
                let exec_incoming = consensus.next().fuse();

                pin_mut!(send_local, exec_incoming);

                block_on(async {
                    select! {
                        local_tx = send_local => {
                            consensus.send_transaction(local_tx.unwrap()).unwrap();
                        },
                        incoming_tx = exec_incoming => {
                            if let Some((tx, peer)) = incoming_tx {
                                // FIXME: we have received transaction tx from Consensus
                                // now we need to execute it on VM
                                println!("From {} got transaction: {}", peer, tx);
                                cpu.set_transaction(tx, peer);
                                cpu.execute_one().unwrap();
                                cpu.print_registers(0, 5);
                            }
                        }
                    }
                });
            }
        }
    }
}
