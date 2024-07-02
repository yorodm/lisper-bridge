use std::{str::from_utf8, sync::Arc, time::Duration};
use crate::{EpcError, EpcRequest, EpcResponse, Handler};
use smol::{io::AsyncReadExt, net::TcpListener};

pub struct EpcServer {
    handler: Arc<dyn Handler>
}


impl<'a> EpcServer {
    pub fn new(handler: Arc<dyn Handler>) -> Self where {
        Self {
            handler: handler.clone()
        }
    }

    pub fn register_listener(&self, listener: TcpListener, timeout: Duration) {
        let handler = self.handler.clone();
        let task = smol::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    match read_request(stream).await {
                        Ok(request) =>  {
                            if let Ok(x) = call_handler(handler.clone(), request).await {

                            }
                        },
                        Err(_) => continue, // Tell client to disconnect
                    };
                } else {
                    continue
                }
            }
        });
        task.detach(); // Send this to the background
    }
}

async fn call_handler(handler: Arc<dyn Handler>, request: EpcRequest) -> Result<EpcResponse, EpcError> {
    handler.handle_call(request);
    todo!()
}

async fn read_request(mut stream: smol::net::TcpStream) -> Result<EpcRequest, EpcError> {
    let mut head = vec![0;6];
    stream.read_exact(&mut head).await.map_err(|r| EpcError::Protocol(r))?;
    let len = usize::from_str_radix(from_utf8(&head).map_err(|_| EpcError::WrongData)?, 16).map_err(|_| EpcError::WrongData)?;
    let mut data: Vec<u8> = vec![0; len];
    stream.read_exact(&mut data).await.map_err(|r| EpcError::Protocol(r))?;
    match lexpr::from_str(from_utf8(&data).map_err(|_| EpcError::WrongData)?).map_err(|_| EpcError::WrongData)? {
        lexpr::Value::Cons(_) => todo!(),
         _ => Err(EpcError::WrongData)
    }
}
