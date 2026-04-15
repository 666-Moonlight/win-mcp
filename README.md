# WinMCP

> Windows Hardware Monitoring MCP Server — 让 AI Agent 读懂你的电脑

**WinMCP** 是一个基于 Rust 开发的 MCP (Model Context Protocol) 服务器，专为 Windows 系统设计，让 AI Agent 能够实时读取本地硬件数据。

---

## 为什么需要这个？

AI Agent 越来越强大，但它们对你的电脑一无所知。WinMCP 填补了这个空白：

- 🔍 **AI 调试助手**：问"为什么程序跑这么慢"，AI 自动查 CPU/内存诊断
- ⚡ **性能监控**：自然语言查询，"找出内存占用最高的 10 个进程"
- 🎮 **AI 应用伴侣**：跑 AI 模型时监控显存/GPU，提示资源不足
- 🖥️ **开发者工具**：给 Cursor / VS Code AI 助手提供硬件上下文

---

## 功能

| 工具 | 说明 |
|:---|:---|
| `get_cpu_usage` | CPU 使用率、核心数、频率、每核心负载 |
| `get_memory_usage` | 内存总量/已用/可用、使用率、Swap 信息 |
| `get_processes` | 进程列表（按 CPU 排序）、PID、名称、内存占用 |

---

## 安装

### 前置要求

- Windows 10/11
- [Rust](https://rustup.rs/) (最新稳定版)
- Claude Desktop 或其他 MCP 兼容客户端

### 编译

```bash
git clone https://github.com/666-Moonlight/win-mcp.git
cd win-mcp
cargo build --release
```

编译产物位于 `target/release/win-mcp.exe`

---

## 配置

### Claude Desktop

编辑 `~/AppData/Roaming/Claude/claude_desktop_config.json`：

```json
{
  "mcpServers": {
    "win-mcp": {
      "command": "C:\\path\\to\\win-mcp.exe"
    }
  }
}
```

### Cursor

在 Cursor 设置中添加 MCP 服务器，路径指向编译后的 `win-mcp.exe`。

---

## 使用示例

安装后，你可以直接问 AI：

> "我的 CPU 使用率是多少？"

> "帮我找出内存占用最高的 5 个进程"

> "为什么这个程序跑这么慢？是内存不够吗？"

---

## 技术细节

- **语言**：Rust — 零运行时依赖，编译后单文件 ~5MB
- **数据源**：`sysinfo` crate + Windows API，性能开销极低
- **协议**：MCP (Model Context Protocol)
- **目标平台**：Windows only (Linux/macOS 未来考虑)

---

## 项目状态

🟡 **MVP 已完成** — 基础功能可用，欢迎试用和反馈

📋 **路线图**
- [ ] GPU 监控 (NVIDIA / AMD)
- [ ] 磁盘 I/O 监控
- [ ] 网络流量监控
- [ ] 告警机制 (阈值触发)
- [ ] Claude Code / Cursor 深度集成示例

---

## 贡献

Issues 和 PR 欢迎！如果你有想加的功能或发现了 Bug，请告诉我们。

---

## License

MIT
