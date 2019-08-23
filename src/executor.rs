// FIXME: `expect`s should be removed and proper errors returned.
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;

use futures::{
    future::Future,
    task::{Context, Poll},
};
use libconsensus_lachesis_rs::tcp_server::{TcpNode, TcpPeer};
use libconsensus_lachesis_rs::{BTreeHashgraph, Event, Node, Swirlds};
use log::debug;
use vm::instruction::Program;
use vm::{Cpu, CpuRevm};

/// The new programs returned by the node.
struct NewPrograms {
    /// The node that produces programs.
    node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>,
    /// The index of the next program.
    program_index: usize,
}

impl Future for NewPrograms {
    type Output = Vec<Program>;

    fn poll(self: Pin<&mut Self>, _cxt: &mut Context) -> Poll<Self::Output> {
        debug!("NewPrograms::<Future>::poll");
        let mut_self = Pin::get_mut(self);
        let events = mut_self
            .node
            .node
            .get_ordered_events()
            .expect("cannot get ordered events");
        let transactions: Vec<Vec<u8>> = events.iter().flat_map(Event::transactions).collect();
        let programs: Vec<_> = transactions
            .into_iter()
            .map(Program::try_from)
            .map(|r| r.expect("cannot convert transaction to program"))
            .collect();
        if programs.len() > mut_self.program_index {
            let last_program_index = mut_self.program_index;
            mut_self.program_index = programs.len();
            Poll::Ready(programs[last_program_index..mut_self.program_index].to_vec())
        } else {
            Poll::Pending
        }
    }
}

impl NewPrograms {
    fn new(node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>) -> Self {
        NewPrograms {
            node,
            program_index: 0,
        }
    }

    /// A wrapper for the `NewPrograms` instance of `Future::poll` that doesn't use `Pin`.
    fn poll(&mut self, cxt: &mut Context) -> Poll<Vec<Program>> {
        debug!("NewPrograms::poll");
        Pin::new(self).poll(cxt)
    }
}

pub struct Executor {
    cpu: CpuRevm,
    new_programs: NewPrograms,
}

impl Future for Executor {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cxt: &mut Context) -> Poll<Self::Output> {
        debug!("Executor::<Future>::poll");
        let mut_self = Pin::get_mut(self);
        match mut_self.new_programs.poll(cxt) {
            Poll::Ready(programs) => {
                for program in programs {
                    mut_self
                        .cpu
                        .execute(program)
                        .expect("cannot execute program");
                }
                Poll::Pending
            }
            _ => Poll::Pending,
        }
    }
}

impl Executor {
    pub fn new(node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>, cpu_memory: usize) -> Self {
        let cpu = CpuRevm::new(cpu_memory).expect("cannot construct a CPU");
        let new_programs = NewPrograms::new(node);
        Executor { cpu, new_programs }
    }
}
