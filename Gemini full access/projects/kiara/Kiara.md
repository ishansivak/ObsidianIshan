# Kiara: The Pro FPGA Synthesis Engine
**Status**: IN DEVELOPMENT (Progress: 15%)
**Core Technology**: Rust, Yosys, nextpnr, openFPGALoader

Kiara is a Rust-native FPGA design suite built to abstract, orchestrate, and program open-source Electronic Design Automation (EDA) tools. By wrapping low-level command-line utilities in type-safe Rust interfaces, Kiara provides a clean, scriptable, and AI-invocable toolchain for hardware-as-code engineering.

---

## Architectural Map
*   **System Specifications**: Defined in [[Kiara_Specification|Kiara Specification]].
*   **Toolchain Wrapping**: Encapsulated atomically in [[Toolchain_Wrapper|Toolchain Wrapper]].
*   **Pipeline Orchestration**: Managed dynamically by the [[Orchestrator|CLI Orchestrator]].
*   **Verification**: Validated using the test suite in [[Orchestrator_Tests|Orchestrator Tests]].
*   **FOSS EDA Deep Dive**: Comparative analysis in [[FPGA_Software_Deep_Dive|FPGA Software Deep Dive]].

---

## Core Engineering Tenets
1.  **Hardware-as-Code**: Treat digital circuits as software. Build, simulate, and compile using standard software engineering pipelines.
2.  **Type Safety**: Leverage Rust's type system to catch configuration, routing, and constraint errors at compile time before uploading to physical silicon.
3.  **Agent-Centric Design**: Maintain machine-readable JSON interfaces (defined in [[Kiara_Specification]]) so local AI copilots can orchestrate synthesis and place-and-route dynamically.
4.  **Zero Bloat**: Bypass massive, multi-gigabyte vendor IDEs. Keep the compilation pipeline lightweight, fast, and fully compatible with headless CI/CD environments.
