---
title: Kurocto Agent
description: Use Kurocto's native AI agent with Kurocto-configured models, tools, profiles, skills, instructions, and MCP servers.
---

# Kurocto Agent

Kurocto Agent is Kurocto's native agent path. It runs in the [Agent Panel](./agent-panel.md) and [Threads Sidebar](./parallel-agents.md#threads-sidebar), uses models configured through [LLM Providers](./llm-providers.md), and integrates with Kurocto's project, editor, terminal, and review surfaces.

Use Kurocto Agent when you want the agent to:

- read and search your project
- edit files
- run terminal commands
- use Kurocto-managed MCP tools
- follow [Agent Profiles](./agent-profiles.md)
- use Kurocto [Skills](./skills.md) and [Instructions](./instructions.md)
- show changes in Kurocto's review UI

## What Kurocto Agent Uses {#what-zed-agent-uses}

| Capability                 | Source of truth                           |
| -------------------------- | ----------------------------------------- |
| Model access               | [LLM Providers](./llm-providers.md)       |
| Panel workflow             | [Agent Panel](./agent-panel.md)           |
| Tool availability          | [Agent Profiles](./agent-profiles.md)     |
| Tool approval behavior     | [Tool Permissions](./tool-permissions.md) |
| Built-in tools             | [Tools](./tools.md)                       |
| External tools             | [MCP](./mcp.md)                           |
| Reusable task instructions | [Skills](./skills.md)                     |
| Always-on instructions     | [Instructions](./instructions.md)         |

## How It Differs from Other Agent Paths {#other-agent-paths}

| Agent path                                | Main difference                                                                              |
| ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| [Kurocto Agent](./zed-agent.md)               | Uses Kurocto's model, tool, profile, skill, instruction, and MCP configuration                   |
| [External Agents](./external-agents.md)   | Use an ACP integration and often own auth, model, tool, and native instruction configuration |
| [Terminal Threads](./terminal-threads.md) | Run a CLI/TUI in a terminal-backed thread; the CLI owns auth and configuration               |

See [Agents](./agents.md) for the full comparison.
