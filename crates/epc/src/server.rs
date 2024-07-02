use std::{str::from_utf8, sync::Arc, time::Duration};
use crate::{EpcError, EpcRequest, Handler};
use smol::{io::AsyncReadExt, net::TcpListener};

pub struct EpcServer<T: Handler> {
    handler: Arc<T>
}


impl<T: Handler> EpcServer<T> {
    pub fn new(handler: T) -> Self {
        Self {
            handler: Arc::new(handler)
        }
    }

    pub fn register_listener(&self, listener: TcpListener, timeout: Duration) {
        let handler = self.handler.clone();
        let task = smol::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    match read_request(stream).await {
                        Ok(_) => todo!(),
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

async fn read_request(mut stream: smol::net::TcpStream) -> Result<EpcRequest, EpcError> {
    let mut head = vec![0;6];
    stream.read_exact(&mut head).await.map_err(|r| EpcError::Protocol(r))?;
    let len = usize::from_str_radix(from_utf8(&head).map_err(|_| EpcError::WrongData)?, 16).map_err(|_| EpcError::WrongData)?;
    let mut data: Vec<u8> = vec![0; len];
    stream.read_exact(&mut data).await.map_err(|r| EpcError::Protocol(r))?;
    match lexpr::from_str(from_utf8(&data).map_err(|_| EpcError::WrongData)?).map_err(|_| EpcError::WrongData)? {
        lexpr::Value::Nil => todo!(),
        lexpr::Value::Null => todo!(),
        lexpr::Value::Bool(_) => todo!(),
        lexpr::Value::Number(_) => todo!(),
        lexpr::Value::Char(_) => todo!(),
        lexpr::Value::String(_) => todo!(),
        lexpr::Value::Symbol(_) => todo!(),
        lexpr::Value::Keyword(_) => todo!(),
        lexpr::Value::Bytes(_) => todo!(),
        lexpr::Value::Cons(_) => todo!(),
        lexpr::Value::Vector(_) => todo!(),
    }
    todo!()
}
