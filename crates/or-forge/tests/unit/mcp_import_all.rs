use or_forge::ForgeRegistry;
use or_mcp::McpTool;
use schemars::schema::{InstanceType, RootSchema, SchemaObject, SingleOrVec};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

#[tokio::test]
async fn import_all_from_mcp_mock_server() {
    let server_url = spawn_mock_mcp_server(
        "forge-mock",
        vec![tool("echo"), tool("inspect")],
    );
    let mut registry = ForgeRegistry::new();

    let summary = registry.import_all_from_mcp(&server_url).await.unwrap();

    assert_eq!(summary.tools_imported, 2);
    assert_eq!(summary.server_name.as_deref(), Some("forge-mock"));
    assert_eq!(summary.tool_names, vec!["echo".to_owned(), "inspect".to_owned()]);

    let result = registry
        .invoke("inspect", serde_json::json!({ "value": 7 }))
        .await
        .unwrap();
    assert_eq!(result["server"], "forge-mock");
    assert_eq!(result["tool"], "inspect");
    assert_eq!(result["arguments"]["value"], 7);
}

fn spawn_mock_mcp_server(server_name: &'static str, tools: Vec<McpTool>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    thread::spawn(move || {
        for stream in listener.incoming().flatten().take(8) {
            let _ = handle_connection(stream, server_name, &tools);
        }
    });
    format!("http://{address}")
}

fn handle_connection(
    mut stream: TcpStream,
    server_name: &str,
    tools: &[McpTool],
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    let request = read_request_body(&mut stream)?;
    let method = request["method"].as_str().unwrap_or_default();
    let id = request["id"].clone();
    let result = match method {
        "initialize" => serde_json::json!({
            "protocolVersion": "2025-11-25",
            "serverInfo": { "name": server_name, "version": "0.1.2" },
            "capabilities": { "tools": {}, "prompts": {}, "resources": {} }
        }),
        "tools/list" => serde_json::json!({ "tools": tools }),
        "tools/call" => serde_json::json!({
            "server": server_name,
            "tool": request["params"]["name"].clone(),
            "arguments": request["params"]["arguments"].clone(),
        }),
        _ => serde_json::json!({}),
    };
    let payload = serde_json::to_vec(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
    .unwrap();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    )?;
    stream.write_all(&payload)
}

fn read_request_body(stream: &mut TcpStream) -> std::io::Result<serde_json::Value> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(body) = extract_body(&buffer) {
            return serde_json::from_slice(body)
                .map_err(|error| std::io::Error::other(error.to_string()));
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "request body not received",
    ))
}

fn extract_body(buffer: &[u8]) -> Option<&[u8]> {
    let marker = b"\r\n\r\n";
    let header_end = buffer.windows(marker.len()).position(|part| part == marker)?;
    let headers = std::str::from_utf8(&buffer[..header_end]).ok()?;
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    let body_start = header_end + marker.len();
    let body_end = body_start + content_length;
    (buffer.len() >= body_end).then_some(&buffer[body_start..body_end])
}

fn tool(name: &str) -> McpTool {
    McpTool {
        name: name.to_owned(),
        description: format!("{name} tool"),
        input_schema: RootSchema {
            meta_schema: None,
            schema: SchemaObject {
                instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
                ..Default::default()
            },
            definitions: Default::default(),
        },
    }
}
