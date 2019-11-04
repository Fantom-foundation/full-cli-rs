use crate::config::{DAGData, EnvDAG};
use ethereum_types::H160;
use evm_rs::transaction::Transaction;
use evm_rs::vm::{Opcode, VM};
use failure::Error;
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::select;
use futures::stream::StreamExt;
use futures::FutureExt;
use libconsensus::Consensus;
use libvm::DistributedVM;

pub struct DVM {
    cpu: Option<VM>,
    algorithm: Option<EnvDAG>,
    stopped: oneshot::Receiver<()>,
}

impl DVM {
    pub fn new() -> (DVM, oneshot::Sender<()>) {
        let (tx, rx) = oneshot::channel();
        return (
            DVM {
                cpu: None,
                algorithm: None,
                stopped: rx,
            },
            tx,
        );
    }

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

    fn serve(self) {
        if let (Some(mut consensus), Some(mut cpu), stopped) =
            (self.algorithm, self.cpu, self.stopped)
        {
            // FIXME: check for exit condition here and do exit when met
            block_on(async {
                let mut stopped = stopped.fuse();
                loop {
                    let mut incoming_tx = consensus.next().fuse();

                    select! {
                        incoming = incoming_tx => {
                            if let Some((tx, peer)) = incoming {
                                // FIXME: we have received transaction tx from Consensus
                                // now we need to execute it on VM
                                println!("From {} got transaction: {}", peer, tx);
                                cpu.set_transaction(tx, peer);
                                cpu.execute_one().unwrap();
                                cpu.print_registers(0, 5);
                            }
                        },
                        res = stopped => {
                            consensus.shutdown().unwrap();
                        }
                    };
                }
            });
        }
    }
}
