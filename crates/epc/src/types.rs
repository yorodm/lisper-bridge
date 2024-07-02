use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EpcError {
    #[error("Peer disconected")]
    Protocol(#[from] std::io::Error),
    #[error("Wrong data received")]
    WrongData,
}

#[derive(Debug)]
pub struct EpcRequest(lexpr::Value);
#[derive(Debug)]
pub struct EpcResponse(lexpr::Value);

impl TryFrom<&str> for EpcRequest {
    type Error = EpcError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(EpcRequest(lexpr::from_str(value).map_err(|_| EpcError::WrongData)?))
    }
}

impl Display for EpcRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for EpcResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
