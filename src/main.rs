use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

mod tools;
use tools::WinMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    let server = WinMcpServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
