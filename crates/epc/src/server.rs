use crate::{types::TimeoutExt, EpcError, EpcRequest, EpcResponse, Handler};
use smol::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use std::{str::from_utf8, sync::Arc, time::Duration};

pub struct EpcServer {
    handler: Arc<dyn Handler>,
}

impl EpcServer {
    pub fn new(handler: Arc<dyn Handler>) -> Self where {
        Self {
            handler: handler.clone(),
        }
    }

    pub fn register_listener(&self, listener: TcpListener, timeout: Duration) {
        let handler = self.handler.clone();
        let task = smol::spawn(async move {
            loop {
                if let Ok((mut stream, _)) = listener.accept().await {
                    match read_request(&mut stream).timeout(timeout).await {
                        Some(completed) => match completed {
                            Ok(request) => {
                                if let Ok(x) = call_handler(handler.clone(), request).await {
                                    let output = format!("{}", x);
                                    stream.write_all(output.as_bytes()).await.ok();
                                    // ignore error
                                }
                            }
                            Err(_) => {
                                stream.shutdown(std::net::Shutdown::Both).ok(); // ignore error
                                break;
                            }
                        },
                        None => continue, // we timed out
                    };
                } else {
                    continue;
                }
            }
        });
        task.detach(); // Send this to the background
    }
}

async fn call_handler(
    handler: Arc<dyn Handler>,
    request: EpcRequest,
) -> Result<EpcResponse, EpcError> {
    match request.kind {
        crate::types::CallType::Call => handler.handle_call(request).await,
        crate::types::CallType::Return => handler.handle_return(request).await,
        crate::types::CallType::ReturnError => handler.handle_return_error(request).await,
        crate::types::CallType::EpcErrorType => handler.handle_epc_error(request).await,
        crate::types::CallType::Methods => handler.handle_methods(request).await,
    }
}

async fn read_request(stream: &mut smol::net::TcpStream) -> Result<EpcRequest, EpcError> {
    let mut head = vec![0; 6];
    stream
        .read_exact(&mut head)
        .await
        .map_err(|r| EpcError::Protocol(r))?;
    let len = usize::from_str_radix(from_utf8(&head).map_err(|_| EpcError::WrongData)?, 16)
        .map_err(|_| EpcError::WrongData)?;
    let mut data: Vec<u8> = vec![0; len];
    stream
        .read_exact(&mut data)
        .await
        .map_err(|r| EpcError::Protocol(r))?;
    match lexpr::from_str(from_utf8(&data).map_err(|_| EpcError::WrongData)?)
        .map_err(|_| EpcError::WrongData)?
    {
        lexpr::Value::Cons(_) => todo!(),
        _ => Err(EpcError::WrongData),
    }
}
