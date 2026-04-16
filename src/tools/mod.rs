use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_router,
};
use sysinfo::System;

// ── Parameter structs ────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyParams {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ProcessParams {
    #[schemars(description = "Maximum number of processes to return (default: 20)")]
    pub limit: Option<usize>,
}

// ── Server ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WinMcpServer {
    tool_router: ToolRouter<Self>,
}

impl WinMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for WinMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tools ────────────────────────────────────────────────────────────────────

#[tool_router]
impl WinMcpServer {
    #[tool(description = "Get current CPU usage percentage, core count, frequency, and per-core load")]
    fn get_cpu_usage(&self, Parameters(_): Parameters<EmptyParams>) -> String {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_all();

        let global_usage = sys.global_cpu_usage();
        let core_count = sys.cpus().len();
        let freq = sys.cpus().first().map(|c| c.frequency()).unwrap_or(0);
        let per_core: Vec<String> = sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| format!("  Core {:2}: {:.1}%", i, cpu.cpu_usage()))
            .collect();

        format!(
            "CPU Usage: {:.1}%\nCores: {}\nFrequency: {} MHz\n\nPer-core load:\n{}",
            global_usage,
            core_count,
            freq,
            per_core.join("\n")
        )
    }

    #[tool(description = "Get memory and swap usage: total, used, available (in MB)")]
    fn get_memory_usage(&self, Parameters(_): Parameters<EmptyParams>) -> String {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let to_mb = |b: u64| b / 1024 / 1024;
        let total = to_mb(sys.total_memory());
        let used = to_mb(sys.used_memory());
        let available = to_mb(sys.available_memory());
        let swap_total = to_mb(sys.total_swap());
        let swap_used = to_mb(sys.used_swap());
        let pct = if total > 0 { used as f64 / total as f64 * 100.0 } else { 0.0 };

        format!(
            "Memory Usage: {:.1}%\nTotal:     {} MB\nUsed:      {} MB\nAvailable: {} MB\n\nSwap Total: {} MB\nSwap Used:  {} MB",
            pct, total, used, available, swap_total, swap_used
        )
    }

    #[tool(description = "List running Windows processes sorted by CPU usage. Use 'limit' to set max count (default 20).")]
    fn get_processes(&self, Parameters(p): Parameters<ProcessParams>) -> String {
        let mut sys = System::new_all();
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let limit = p.limit.unwrap_or(20);
        let mut procs: Vec<_> = sys.processes().values().collect();
        procs.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

        let header = format!("{:<8} {:>7} {:>10}  {}", "PID", "CPU%", "MEM(MB)", "Name");
        let sep = "-".repeat(50);
        let rows: Vec<String> = procs.iter().take(limit).map(|p| {
            format!(
                "{:<8} {:>6.1}% {:>8} MB  {}",
                p.pid().as_u32(),
                p.cpu_usage(),
                p.memory() / 1024 / 1024,
                p.name().to_string_lossy()
            )
        }).collect();

        format!("Top {} processes by CPU:\n{}\n{}\n{}", limit, header, sep, rows.join("\n"))
    }
}

// ── ServerHandler ────────────────────────────────────────────────────────────

impl ServerHandler for WinMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Windows hardware monitoring MCP server. Tools: get_cpu_usage, get_memory_usage, get_processes")
    }
}
