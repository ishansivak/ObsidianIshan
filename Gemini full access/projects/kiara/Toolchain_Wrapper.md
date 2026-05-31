# Kiara: Toolchain Wrapper

## Overview
Atomic Rust wrapper for FPGA toolchain commands (e.g., synthesis, place-and-route).

```rust
use std::process::Command;

#[derive(Clone)]
pub struct ToolchainWrapper {
    pub command: String,
    pub args: Vec<String>,
}

impl ToolchainWrapper {
    pub fn run(&self) -> Result<(), String> {
        let status = Command::new(&self.command)
            .args(&self.args)
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Command failed with status: {}", status))
        }
    }
}
```

## Concrete Example (Yosys)
```rust
pub struct YosysWrapper {
    pub wrapper: ToolchainWrapper,
}

impl YosysWrapper {
    pub fn synthesize(&self, file: &str) -> Result<(), String> {
        // Run yosys synthesis with the provided file
        let cmd = ToolchainWrapper {
            command: self.wrapper.command.clone(),
            args: vec![file.to_string()],
        };
        cmd.run()
    }
}
```

## Concrete Example (Nextpnr)
```rust
pub struct NextpnrWrapper {
    pub wrapper: ToolchainWrapper,
}

impl NextpnrWrapper {
    pub fn place_and_route(&self, input: &str, output: &str) -> Result<(), String> {
        // Run nextpnr place-and-route
        let cmd = ToolchainWrapper {
            command: self.wrapper.command.clone(),
            args: vec![input.to_string(), "-o".to_string(), output.to_string()],
        };
        cmd.run()
    }
}
```

## Concrete Example (BitstreamGenerator)
```rust
pub struct BitstreamGenerator {
    pub wrapper: ToolchainWrapper,
}

impl BitstreamGenerator {
    pub fn generate(&self, input: &str, output: &str) -> Result<(), String> {
        // Run bitstream generation
        let cmd = ToolchainWrapper {
            command: self.wrapper.command.clone(),
            args: vec![input.to_string(), "--bitstream".to_string(), output.to_string()],
        };
        cmd.run()
    }
}
```
