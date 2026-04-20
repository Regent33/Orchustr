use or_mcp::{
    JsonRpcMessage, McpTask, McpTool, McpTransport, NexusClient, NexusClientTrait, NexusServer,
    NexusServerTrait, ServerCard,
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, mpsc};

#[derive(Debug)]
struct ChannelTransport {
    incoming: Mutex<mpsc::UnboundedReceiver<JsonRpcMessage>>,
    outgoing: mpsc::UnboundedSender<JsonRpcMessage>,
}

impl ChannelTransport {
    fn pair() -> (Self, Self) {
        let (client_to_server_tx, client_to_server_rx) = mpsc::unbounded_channel();
        let (server_to_client_tx, server_to_client_rx) = mpsc::unbounded_channel();
        (
            Self {
                incoming: Mutex::new(server_to_client_rx),
                outgoing: client_to_server_tx,
            },
            Self {
                incoming: Mutex::new(client_to_server_rx),
                outgoing: server_to_client_tx,
            },
        )
    }
}

impl McpTransport for ChannelTransport {
    async fn send_message(&self, msg: &JsonRpcMessage) -> Result<(), or_mcp::McpError> {
        self.outgoing
            .send(msg.clone())
            .map_err(|error| or_mcp::McpError::Transport(error.to_string()))
    }

    async fn receive_message(&self) -> Result<JsonRpcMessage, or_mcp::McpError> {
        self.incoming
            .lock()
            .await
            .recv()
            .await
            .ok_or_else(|| or_mcp::McpError::Transport("channel closed".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct EchoArgs {
    text: String,
}

#[tokio::test]
async fn nexus_client_and_server_round_trip_tools_and_tasks() {
    let (client_transport, server_transport) = ChannelTransport::pair();
    let mut server = NexusServer::new(
        server_transport,
        ServerCard {
            name: "orchustr".into(),
            version: "0.1.1".into(),
            protocol_version: "2025-11-25".into(),
        },
    );
    server
        .register_tool_handler(
            McpTool {
                name: "echo".into(),
                description: "Echoes text".into(),
                input_schema: schema_for!(EchoArgs),
            },
            |args| async move { Ok(serde_json::json!({ "echo": args["text"] })) },
        )
        .await
        .expect("tool should register");
    server
        .register_task(McpTask {
            id: "task-1".into(),
            status: "running".into(),
            expires_at: None,
        })
        .await;

    let server_task = tokio::spawn(async move { server.serve().await });
    let client = NexusClient::new(client_transport);

    let initialized = client
        .initialize()
        .await
        .expect("initialize should succeed");
    assert_eq!(initialized["protocolVersion"], "2025-11-25");

    let tools = client
        .list_tools()
        .await
        .expect("tools/list should succeed");
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "echo");

    let result = client
        .invoke_tool("echo", serde_json::json!({ "text": "hello" }))
        .await
        .expect("tools/call should succeed");
    assert_eq!(result["echo"], "hello");

    let task = client.get_task("task-1").await.expect("task should load");
    assert_eq!(task.status, "running");

    let shutdown = client
        .send("shutdown", serde_json::json!({}))
        .await
        .expect("shutdown should succeed");
    assert_eq!(shutdown["shutdown"], true);

    server_task
        .await
        .expect("server task should join")
        .expect("server should exit cleanly");
}
