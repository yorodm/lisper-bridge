use smol::lock::RwLock;
use smol::net::{TcpListener, TcpStream};
use smol::Executor;
use std::io::Result;
use std::net::Ipv4Addr;
use std::sync::Arc;

use super::protocol::{EpcRequest, EpcResponse};

pub struct EpcHandler{

}
pub struct EpcServer {
    socket: TcpListener,
}

impl EpcServer {
    async fn new(addr: Ipv4Addr, port: u16) -> Result<EpcServer> {
        let socket = TcpListener::bind((addr, port));
        Ok(EpcServer { socket })
    }

    async fn run(&self, ex: &Arc<Executor<'static>>) -> Result<()> {
        loop {
            let (mut stream, _) = self.socket.accept().await?;
            ex.spawn( async move {
                if let Ok(request) = self.read_request(&mut stream).await{
                    if let Ok(response) = self.dispatch(request).await {
                        self.send_response(response).await
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                };
            }).detach();
        }
    }

    async fn read_request(&self, stream: &mut TcpStream) -> Result<EpcRequest> {
        stream.
    }

    async fn dispatch(&self, request: EpcRequest) -> Result<EpcResponse> {
        todo!()
    }

    async fn send_response(&self, response: EpcResponse)->Result<()> {
        todo!()
    }
}
