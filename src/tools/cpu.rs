use mcp_server::tool::{Tool, ToolInput, ToolOutput};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use crate::types::CpuInfo;

#[derive(Debug, Deserialize)]
pub struct CpuUsageInput {}

#[derive(Debug, Serialize)]
pub struct CpuUsageOutput {
    pub info: CpuInfo,
    pub per_core_usage: Vec<f32>,
}

pub fn create_cpu_usage_tool() -> Tool {
    Tool::new(
        "get_cpu_usage",
        "Get CPU usage information for the current system. Returns overall CPU usage percentage, per-core breakdown, and basic CPU metadata.",
        ToolInput::Empty,
        |_params: CpuUsageInput, _context| {
            let mut sys = System::new_all();
            sys.refresh_cpu_all();
            std::thread::sleep(std::time::Duration::from_millis(200));
            sys.refresh_cpu_all();

            let cpus = sys.cpus();
            let overall_usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;

            let per_core_usage: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

            let cpu_name = cpus.first()
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let frequency = cpus.first()
                .map(|c| c.frequency())
                .unwrap_or(0);

            let info = CpuInfo {
                name: cpu_name,
                usage_percent: overall_usage,
                core_count: cpus.len(),
                frequency_mhz: frequency,
            };

            let output = CpuUsageOutput {
                info,
                per_core_usage,
            };

            ToolOutput::Success(serde_json::to_value(output).unwrap_or_default())
        },
    )
}
