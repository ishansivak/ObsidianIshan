# Kiara Core Specification

## 1. Goal
A Rust-native, AI-invocable FPGA design suite orchestrating FOSS EDA tools (Yosys, nextpnr, openFPGALoader).

## 2. Core Contracts
All EDA tools must implement the `ToolchainCommand` trait. Direct shell execution is prohibited.

### Trait Definition
```rust
trait ToolchainCommand {
    fn execute(&self, config: CommandConfig) -> Result<ExecutionResult>;
}
```

### Data Contracts (JSON)
#### CommandConfig
```json
{
  "tool_name": "string",
  "executable_path": "string",
  "arguments": ["string"],
  "environment": {"key": "value"},
  "working_directory": "string"
}
```

#### ExecutionResult
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

## 3. Orchestrator
The `ProjectOrchestrator` manages the dependency graph (DAG) between `ToolchainCommand` implementations. Pipeline configurations must be serializable (JSON/YAML) to enable AI-driven workflow generation.

## 4. Phase 1 Roadmap
1. Define `clap` CLI structure.
2. Implement `ToolchainCommand` trait.
3. Create wrappers for `yosys`, `nextpnr`, `openFPGALoader`.
4. Implement `ProjectOrchestrator` (DAG dependency tracking).
