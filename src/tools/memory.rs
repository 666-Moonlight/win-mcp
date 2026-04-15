use mcp_server::tool::{Tool, ToolInput, ToolOutput};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use crate::types::MemoryInfo;

#[derive(Debug, Deserialize)]
pub struct MemoryUsageInput {}

#[derive(Debug, Serialize)]
pub struct MemoryUsageOutput {
    pub memory: MemoryInfo,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

pub fn create_memory_usage_tool() -> Tool {
    Tool::new(
        "get_memory_usage",
        "Get memory (RAM) usage information for the current system. Returns total, used, available memory in bytes and percentage.",
        ToolInput::Empty,
        |_params: MemoryUsageInput, _context| {
            let sys = System::new_all();

            let total = sys.total_memory();
            let used = sys.used_memory();
            let available = sys.available_memory();
            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as f32
            } else {
                0.0
            };

            let swap_total = sys.total_swap();
            let swap_used = sys.used_swap();

            let output = MemoryUsageOutput {
                memory: MemoryInfo {
                    total_bytes: total,
                    used_bytes: used,
                    available_bytes: available,
                    usage_percent,
                },
                swap_total_bytes: swap_total,
                swap_used_bytes: swap_used,
            };

            ToolOutput::Success(serde_json::to_value(output).unwrap_or_default())
        },
    )
}
