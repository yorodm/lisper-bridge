use pin_project_lite::pin_project;
use smol::Timer;
use std::{
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EpcError {
    #[error("Peer disconected")]
    Protocol(#[from] std::io::Error),
    #[error("Wrong data received")]
    WrongData,
}

#[derive(Debug)]
pub(crate) enum CallType {
    Call,
    Return,
    ReturnError,
    EpcErrorType,
    Methods,
}

impl TryFrom<&lexpr::Value> for CallType {
    type Error = EpcError;

    fn try_from(value: &lexpr::Value) -> Result<Self, Self::Error> {
        match value {
            lexpr::Value::String(x) => parse_call_type(x),
            lexpr::Value::Symbol(x) => parse_call_type(x),
            _ => Err(EpcError::WrongData),
        }
    }
}

fn parse_call_type(data: &str) -> Result<CallType, EpcError> {
    match data {
        "call" => Ok(CallType::Call),
        "return" => Ok(CallType::Return),
        "return-error" => Ok(CallType::ReturnError),
        "epc-error" => Ok(CallType::EpcErrorType),
        "methods" => Ok(CallType::Methods),
        _ => Err(EpcError::WrongData),
    }
}

#[derive(Debug)]
pub struct MessageBody {
    pub uuid: u64,
    pub args: lexpr::Value,
}

impl TryFrom<&lexpr::Value> for MessageBody {
    type Error = EpcError;

    fn try_from(value: &lexpr::Value) -> Result<Self, Self::Error> {
        match value {
            lexpr::Value::Cons(body) => {
                let uuid = match body.car() {
                    lexpr::Value::Number(uid) => uid.as_u64().ok_or(EpcError::WrongData),
                    _ => Err(EpcError::WrongData),
                }?;
                Ok(MessageBody {
                    uuid: uuid,
                    args: body.cdr().to_owned(),
                })
            }
            _ => Err(EpcError::WrongData),
        }
    }
}

#[derive(Debug)]
pub struct EpcRequest {
    pub(crate) kind: CallType,
    pub(crate) body: MessageBody,
}

#[derive(Debug)]
pub struct EpcResponse(lexpr::Value);

impl TryFrom<lexpr::Value> for EpcRequest {
    type Error = EpcError;

    fn try_from(value: lexpr::Value) -> Result<Self, Self::Error> {
        match value {
            lexpr::Value::Cons(cons) => {
                let call_type: CallType = cons.car().try_into()?;
                Ok(EpcRequest {
                    kind: call_type,
                    body: cons.cdr().try_into()?,
                })
            }
            _ => Err(EpcError::WrongData),
        }
    }
}

impl Display for EpcRequest {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for EpcResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pin_project! {
    pub (crate) struct Timeout<F: Future> {
        #[pin]
        future: F,
        #[pin]
        timer: Timer,
    }
}

pub(crate) trait TimeoutExt: Future {
    fn timeout(self, after: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout {
            future: self,
            timer: Timer::after(after),
        }
    }
}

impl<Fut: Future> TimeoutExt for Fut {}

impl<Fut: Future> Future for Timeout<Fut> {
    type Output = Option<Fut::Output>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.timer.poll(ctx).is_ready() {
            return Poll::Ready(None);
        }

        if let Poll::Ready(output) = this.future.poll(ctx) {
            return Poll::Ready(Some(output));
        }

        Poll::Pending
    }
}
