# Reddit (r/programming / r/MachineLearning) 发帖内容

---

**Title:**
[Open Source] WinMCP — MCP Server for Windows Hardware Monitoring (Rust, 5MB binary)

---

**Body:**

## WinMCP — Let AI Agents Read Your Windows System

I built a Rust-based MCP (Model Context Protocol) server for Windows that lets AI agents query real-time hardware data.

GitHub: https://github.com/666-Moonlight/win-mcp

---

## What it does

```
> "What's my CPU usage right now?"
> "Find the top 5 processes by memory"
> "Why is my program running so slow? Is it memory?"
```

Three tools so far:
- `get_cpu_usage` — CPU %, cores, frequency, per-core load
- `get_memory_usage` — total/used/available RAM, swap
- `get_processes` — process list sorted by CPU/memory

---

## Why

- Linux monitoring MCPs exist. Windows? Nearly nothing.
- Real use cases: dev debugging, Cursor/Claude Code integration, AI app performance assistants
- Rust: ~5MB binary, zero runtime dependencies, fast polling

---

## Quick Start

```bash
git clone https://github.com/666-Moonlight/win-mcp.git
cd win-mcp
cargo build --release

# Add to Claude Desktop config (~/.config/Claude/claude_desktop_config.json):
# "mcpServers": { "win-mcp": { "command": "path/to/win-mcp.exe" } }
```

---

## Roadmap (feedback wanted!)

- [ ] GPU monitoring (NVIDIA / AMD)
- [ ] Disk I/O stats
- [ ] Network traffic
- [ ] Threshold alerts

Would love feedback on which direction to prioritize — or if you think this is even useful. Issues and PRs welcome!

---

**Tags:** #MCP #Rust #Windows #AI #DevTools
