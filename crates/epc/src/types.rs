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
pub enum CallType {

}

#[derive(Debug)]
pub struct EpcRequest{
    kind : CallType,
    args: lexpr::Value
}

impl TryFrom<&lexpr::Value> for CallType {
    type Error = EpcError;

    fn try_from(value: &lexpr::Value) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct EpcResponse(lexpr::Value);

impl TryFrom<lexpr::Value> for EpcRequest {
    type Error = EpcError;

    fn try_from(value: lexpr::Value) -> Result<Self, Self::Error> {
        match value {
            lexpr::Value::Cons(cons) =>  {
                let call_type : CallType = cons.car().try_into()?;
                Ok(EpcRequest{ kind: call_type, args: cons.cdr().to_owned()})
            },
            _ => Err(EpcError::WrongData)
        }
    }
}

impl Display for EpcRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for EpcResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
