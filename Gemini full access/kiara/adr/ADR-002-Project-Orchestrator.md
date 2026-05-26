# ADR-002: Project Orchestrator

## Status
Proposed

## Context
A single FPGA design flow requires chaining multiple tools (synthesis -> P&R -> bitstream). The system needs a component to manage dependency graphs, state, and pipeline execution.

## Decision
Implement a `ProjectOrchestrator` component that defines the workflow pipeline.

## Consequences
- **Positive**: Enables complex, multi-step automated builds. Provides a single point for pipeline monitoring, cancellation, and artifact management.
- **Negative**: Increases architectural complexity; requires robust state management.

## Requirements
- Orchestrator must consume `ToolchainCommand` implementations.
- Pipeline definition must be serializable (e.g., YAML/JSON) to allow AI agents to generate or modify workflows.
- Orchestrator must track dependencies between tool steps (e.g., P&R cannot start until synthesis artifact is generated).
