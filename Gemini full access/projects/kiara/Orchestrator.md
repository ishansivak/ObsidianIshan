# Kiara: CLI Orchestrator

## Overview
The Orchestrator ties together atomic wrappers (YosysWrapper, NextpnrWrapper, BitstreamGenerator) into a dynamic, dependency-aware FPGA synthesis and implementation pipeline, as defined in [[ADR-002-Project-Orchestrator]].

## Testing
See [[Orchestrator_Tests]] for the unit test suite.

## Implementation (Refactored to align with ADR-002)
```rust
use std::collections::HashSet;

pub trait ToolchainCommand {
    fn execute(&self) -> Result<(), String>;
    fn dependencies(&self) -> Vec<String>;
    fn name(&self) -> String;
}

pub struct ProjectOrchestrator {
    commands: Vec<Box<dyn ToolchainCommand>>,
    completed_steps: HashSet<String>,
}

impl ProjectOrchestrator {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            completed_steps: HashSet::new(),
        }
    }

    // Agent-centric pipeline loading (JSON/YAML)
    pub fn from_json(json_data: &str) -> Result<Self, String> {
        // Implementation for deserializing pipeline workflow
        todo!("Implement JSON deserialization for pipeline definition")
    }

    pub fn add_command(&mut self, cmd: Box<dyn ToolchainCommand>) {
        self.commands.push(cmd);
    }

    pub fn run_pipeline(&mut self) -> Result<(), String> {
        for cmd in &self.commands {
            // Verify dependencies exist in completed_steps
            for dep in cmd.dependencies() {
                if !self.completed_steps.contains(&dep) {
                    return Err(format!("Dependency '{}' not met for step '{}'", dep, cmd.name()));
                }
            }
            // Execute
            cmd.execute()?;
            // Mark step as complete
            self.completed_steps.insert(cmd.name());
        }
        Ok(())
    }
}
```
