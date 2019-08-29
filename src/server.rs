//use std::net::SocketAddr;
//use std::sync::Arc;
//
//use failure::Fail;
//use futures;
//
use crate::constants::LOCALHOST;
//use crate::executor::Executor;
//
//const SOCKET_BUFFER_SIZE: usize = 4096;
//
//#[derive(Debug, Fail)]
//pub enum Error {
//    #[fail(display = "Socket address parse error: {}", _0)]
//    Parse(std::net::AddrParseError),
//    #[fail(display = "Socket bind error: {}", _0)]
//    Bind(std::io::Error),
//    #[fail(display = "Accept error: {}", _0)]
//    Accept(std::io::Error),
//}
//
//pub struct Server {
//    node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>,
//    port: usize,
//}
//
//impl Server {
//    pub fn new(port: usize, node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>) -> Server {
//        Server { node, port }
//    }
//
//    pub async fn run(&self, cpu_memory: usize) -> (Result<(), Error>, ()) {
//        let g = self.run_queue_consumer(cpu_memory);
//        futures::join!(f, g)
//    }
//
//    async fn run_queue_consumer(&self, cpu_memory: usize) {
//        Executor::new(self.node.clone(), cpu_memory).await;
//    }
//}
