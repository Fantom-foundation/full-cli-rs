// FIXME: `expect`s should be removed and proper errors returned.
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::{
    future::Future,
    task::{Context, Poll},
};
use libconsensus_lachesis_rs::tcp_server::{TcpNode, TcpPeer};
use libconsensus_lachesis_rs::{BTreeHashgraph, Event, Node, Swirlds};
use log::debug;
use vm::instruction::Program;
use vm::CpuRevm;

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
            return Poll::Ready(programs[last_program_index..mut_self.program_index].to_vec());
        }
        Poll::Pending
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
        if let Poll::Ready(programs) = mut_self.new_programs.poll(cxt) {
            for program in programs {
                mut_self
                    .cpu
                    .execute(program)
                    .expect("cannot execute program");
            }
        }
        let waker = cxt.waker().clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(200));
            waker.wake();
        });
        Poll::Pending
    }
}

impl Executor {
    pub fn new(node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>, cpu_memory: usize) -> Self {
        let cpu = CpuRevm::new(cpu_memory).expect("cannot construct a CPU");
        let new_programs = NewPrograms::new(node);
        Executor { cpu, new_programs }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::thread;
    use std::time::{Duration, Instant};

    use futures::{
        self,
        executor::block_on,
        future::Future,
        task::{Context, Poll},
    };
    use log::debug;

    struct Delay {
        duration: Duration,
        start: Instant,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cxt: &mut Context) -> Poll<Self::Output> {
            let myself = Pin::get_mut(self);
            if myself.finished() {
                debug!("Poll::Ready");
                return Poll::Ready(());
            }
            debug!("Poll::Pending");
            let waker = cxt.waker().clone();
            // Schedule a poll in half a second.
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(500));
                waker.wake();
            });
            Poll::Pending
        }
    }

    impl Delay {
        fn new(duration: Duration) -> Self {
            let start = Instant::now();
            Delay { duration, start }
        }

        fn finished(&self) -> bool {
            self.start.elapsed() >= self.duration
        }
    }

    #[test]
    fn should_delay_for_5_seconds() {
        env_logger::init();
        let start = Instant::now();
        let duration = Duration::from_secs(5);
        debug!("start: {:?}, duration: {:?}", start, duration);
        block_on(async {
            debug!("Delay::new");
            Delay::new(duration).await;
        });
        assert!(start.elapsed() >= duration);
    }
}
