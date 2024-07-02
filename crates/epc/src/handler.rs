use crate::EpcError;
use crate::EpcRequest;
use crate::EpcResponse;
use async_trait::async_trait;
#[async_trait]
pub trait Handler {
    async fn handle_call(request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_return(request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_return_error(request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_epc_error(request: EpcRequest) -> Result<EpcResponse, EpcError>;
    async fn handle_methods(request: EpcRequest) -> Result<EpcResponse, EpcError>;
}
