# Kiara Project Master Specification

## 1. Overview
Kiara is a Rust-native, AI-invocable FPGA design suite built to abstract and orchestrate FOSS EDA tools (Yosys, nextpnr, openFPGALoader).

## 2. Architectural Foundation
- **ADR-001 (Toolchain Wrapper)**: All EDA tools must be wrapped in a `ToolchainCommand` trait. Direct shell execution is prohibited.
- **ADR-002 (Project Orchestrator)**: A centralized component manages dependency graphs and pipeline state.

## 3. System Contracts (Data Structures)

### CommandConfig (JSON)
Used to pass parameters to `ToolchainCommand` implementations.
```json
{
  "tool_name": "string",
  "executable_path": "string",
  "arguments": ["string"],
  "environment": {"key": "value"},
  "working_directory": "string"
}
```

### ExecutionResult (JSON)
Returned by `ToolchainCommand` after task completion.
```json
{
  "success": "boolean",
  "exit_code": "integer",
  "stdout": "string",
  "stderr": "string",
  "artifacts": ["string"],
  "duration_ms": "integer"
}
```

## 4. Phase 1 Roadmap: CLI & Wrapper Structure
1.  **CLI Interface**: Define `clap` structure for command routing (e.g., `kiara synth`, `kiara pnr`, `kiara load`).
2.  **Trait Definition**: Implement `ToolchainCommand` trait.
3.  **Wrapper Implementation**: Create specific wrappers for `yosys`, `nextpnr`, and `openFPGALoader` implementing the trait.
4.  **Orchestrator Stub**: Implement basic dependency tracking (DAG) to ensure synthesis completes before P&R.

## 5. Implementation Constraints
- **NO IMPLEMENTATION CODE**: This specification defines interfaces and contracts.
- **Language**: Rust (Edition 2021).
- **Dependencies**: `clap`, `serde`, `serde_json`, `anyhow`.
