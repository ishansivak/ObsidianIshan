# Kiara: CLI Orchestrator

## Overview
The Orchestrator ties together the atomic wrappers (`YosysWrapper`, `NextpnrWrapper`, `BitstreamGenerator`) into a unified FPGA synthesis and implementation pipeline.

## Testing
See [[Orchestrator_Tests]] for the unit test suite.

## Implementation
```rust
use crate::wrappers::{YosysWrapper, NextpnrWrapper, BitstreamGenerator};

pub struct KiaraOrchestrator {
    pub yosys: YosysWrapper,
    pub nextpnr: NextpnrWrapper,
    pub bitgen: BitstreamGenerator,
}

impl KiaraOrchestrator {
    pub fn run_pipeline(&self, source: &str, netlist: &str, bitstream: &str) -> Result<(), String> {
        // 1. Synthesis
        self.yosys.synthesize(source)?;
        
        // 2. Place and Route
        self.nextpnr.place_and_route(netlist, "temp.json")?;
        
        // 3. Bitstream Generation
        self.bitgen.generate("temp.json", bitstream)?;
        
        Ok(())
    }
}
```
