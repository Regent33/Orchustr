pub mod application;
pub mod domain;
pub mod infra;

pub use application::client::NexusClient;
pub use application::orchestrators::JsonRpcOrchestrator;
pub use application::server::NexusServer;
pub use domain::contracts::{McpTransport, NexusClientTrait, NexusServerTrait};
pub use domain::entities::{
    JsonRpcErrorDetail, JsonRpcErrorResponse, JsonRpcId, JsonRpcMessage, JsonRpcNotification,
    JsonRpcPacket, JsonRpcRequest, JsonRpcSuccessResponse, McpPrompt, McpResource, McpTask,
    McpTool, ServerCard,
};
pub use domain::errors::McpError;
pub use infra::http_transport::StreamableHttpTransport;
pub use infra::stdio_transport::StdioTransport;
