//// FIXME: `expect`s should be removed and proper errors returned.
//use std::convert::TryFrom;
//use std::pin::Pin;
//use std::sync::Arc;
//
//use futures::{
//    future::Future,
//    task::{Context, Poll},
//};
//
//use log::debug;
//use vm::instruction::Program;
//use vm::CpuRevm;
//
///// The new programs returned by the node.
////    struct NewPrograms {
////    /// The node that produces programs.
////    /// The index of the next program.
////    program_index: usize,
////}
//
//impl Future for NewPrograms {
//    type Output = Vec<Program>;
//
//    fn poll(self: Pin<&mut Self>, _cxt: &mut Context) -> Poll<Self::Output> {
//
//    }
//}
//
//impl NewPrograms {
//
//    /// A wrapper for the `NewPrograms` instance of `Future::poll` that doesn't use `Pin`.
//    fn poll(&mut self, cxt: &mut Context) -> Poll<Vec<Program>> {
//        debug!("NewPrograms::poll");
//        Pin::new(self).poll(cxt)
//    }
//}
//
//pub struct Executor {
//    cpu: CpuRevm,
//    new_programs: NewPrograms,
//}
//
//impl Future for Executor {
//    type Output = ();
//
//    fn poll(self: Pin<&mut Self>, cxt: &mut Context) -> Poll<Self::Output> {
//        debug!("Executor::<Future>::poll");
//        let mut_self = Pin::get_mut(self);
//        match mut_self.new_programs.poll(cxt) {
//            Poll::Ready(programs) => {
//                for program in programs {
//                    mut_self
//                        .cpu
//                        .execute(program)
//                        .expect("cannot execute program");
//                }
//                Poll::Pending
//            }
//            _ => Poll::Pending,
//        }
//    }
//}
//
//impl Executor {
//    pub fn new(node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>, cpu_memory: usize) -> Self {
//        let cpu = CpuRevm::new(cpu_memory).expect("cannot construct a CPU");
//        let new_programs = NewPrograms::new(node);
//        Executor { cpu, new_programs }
//    }
//}
