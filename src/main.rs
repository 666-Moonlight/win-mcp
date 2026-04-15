use mcp_server::server::{Builder, Server};
use std::sync::Arc;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod tools;
mod types;

fn build_server() -> Server {
    let mut builder = Builder::stdio();

    builder.name("WinMCP")
        .version(env!("CARGO_PKG_VERSION"))
        .description("Windows hardware monitoring MCP server for AI agents");

    builder.tool(tools::create_cpu_usage_tool());
    builder.tool(tools::create_memory_usage_tool());
    builder.tool(tools::create_processes_tool());

    builder.build().expect("Failed to build MCP server")
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting WinMCP v{}", env!("CARGO_PKG_VERSION"));
    info!("Windows Hardware Monitoring MCP Server for AI Agents");

    let server = build_server();
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        if let Err(e) = server.run().await {
            error!("Server error: {}", e);
            std::process::exit(1);
        }
    });
}
