use crate::EpcError;
use crate::EpcRequest;
use crate::EpcResponse;
use async_trait::async_trait;
#[async_trait]
pub trait Handler: Send + Sync{
    async fn handle_call(&self, request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_return(&self, request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_return_error(&self, request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_epc_error(&self, request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_methods(&self, request: EpcRequest) -> Result<EpcResponse, EpcError>;
}
