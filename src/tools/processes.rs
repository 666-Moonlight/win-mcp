use mcp_server::tool::{Tool, ToolInput, ToolOutput};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use sysinfo::{System, ProcessesToUpdate, Pid};
use crate::types::ProcessInfo;

#[derive(Debug, Deserialize)]
pub struct ProcessesInput {
    #[serde(default = "default_limit")]
    pub limit: Option<usize>,
}

fn default_limit() -> usize { 20 }

#[derive(Debug, Serialize)]
pub struct ProcessesOutput {
    pub processes: Vec<ProcessInfo>,
    pub total_count: usize,
}

pub fn create_processes_tool() -> Tool {
    Tool::new(
        "get_processes",
        "Get a list of running processes on the system, sorted by CPU or memory usage. Returns top N processes (default 20). Each process includes PID, name, CPU%, memory bytes, and status.",
        ToolInput::Json,
        |params: Value, _context| {
            let limit = serde_json::from_value::<ProcessesInput>(params.clone())
                .map(|p| p.limit.unwrap_or(20))
                .unwrap_or(20);

            let mut sys = System::new_all();
            sys.refresh_processes_specifics(ProcessesToUpdate::All, true);

            let mut processes: Vec<ProcessInfo> = sys.processes()
                .iter()
                .map(|(pid, process)| {
                    ProcessInfo {
                        pid: pid.as_u32(),
                        name: process.name().to_string_lossy().to_string(),
                        cpu_percent: process.cpu_usage(),
                        memory_bytes: process.memory(),
                        status: format!("{:?}", process.status()),
                    }
                })
                .collect();

            // Sort by CPU usage descending, then by memory
            processes.sort_by(|a, b| {
                b.cpu_percent.partial_cmp(&a.cpu_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let total_count = processes.len();
            processes.truncate(limit);

            let output = ProcessesOutput {
                processes,
                total_count,
            };

            ToolOutput::Success(serde_json::to_value(output).unwrap_or_default())
        },
    )
}
