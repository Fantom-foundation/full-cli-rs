use std::net::SocketAddr;
use std::sync::Arc;

use failure::Fail;
use futures;
use libconsensus_lachesis_rs::tcp_server::{TcpNode, TcpPeer};
use libconsensus_lachesis_rs::{BTreeHashgraph, Node, Swirlds};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener; //use tokio::net::TcpStream;

use crate::constants::LOCALHOST;
use crate::executor::Executor;

const SOCKET_BUFFER_SIZE: usize = 4096;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Socket address parse error: {}", _0)]
    Parse(std::net::AddrParseError),
    #[fail(display = "Socket bind error: {}", _0)]
    Bind(std::io::Error),
    #[fail(display = "Accept error: {}", _0)]
    Accept(std::io::Error),
}

pub struct Server {
    node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>,
    port: usize,
}

impl Server {
    pub fn new(port: usize, node: Arc<TcpNode<Swirlds<TcpPeer, BTreeHashgraph>>>) -> Server {
        Server { node, port }
    }

    pub async fn run(&self, cpu_memory: usize) -> (Result<(), Error>, ()) {
        let f = self.run_server();
        let g = self.run_queue_consumer(cpu_memory);
        futures::join!(f, g)
    }

    async fn run_queue_consumer(&self, cpu_memory: usize) {
        Executor::new(self.node.clone(), cpu_memory).await;
    }

    async fn run_server(&self) -> Result<(), Error> {
        let port = self.port;
        let address: SocketAddr = format!("{}:{}", LOCALHOST, port)
            .parse()
            .map_err(Error::Parse)?;
        let mut listener = TcpListener::bind(&address).map_err(Error::Bind)?;
        let node_ref = Arc::new(self.node.clone());
        loop {
            let node_ref = node_ref.clone();
            let (mut socket, _) = listener.accept().await.map_err(Error::Accept)?;
            tokio::spawn(async move {
                let mut buf: [u8; SOCKET_BUFFER_SIZE] = [0; SOCKET_BUFFER_SIZE];
                // TODO: `Error` handling
                let len = socket.read(&mut buf).await.expect("socket read failed");
                node_ref.node.add_transaction(buf[..len].to_vec()).unwrap();
            });
        }
    }
}
