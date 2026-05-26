# ADR-001: Toolchain Wrapper Interface

## Status
Proposed

## Context
Kiara must programmatically invoke heterogeneous FOSS EDA tools (Yosys, nextpnr, openFPGALoader). Direct shell execution is brittle, untestable, and difficult for an AI agent to monitor.

## Decision
Implement an abstract `ToolchainCommand` trait in Rust.

## Consequences
- **Positive**: Decouples Kiara core from tool-specific CLI syntax. Enables mockable interfaces for testing. Standardizes logging and error handling.
- **Negative**: Adds minor boilerplate for each new tool wrapper.

## Requirements
- `trait ToolchainCommand { fn execute(&self, config: CommandConfig) -> Result<ExecutionResult>; }`
- `struct CommandConfig`: Serialized parameters.
- `struct ExecutionResult`: Standardized success/failure and artifact paths.
