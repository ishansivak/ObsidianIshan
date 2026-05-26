# Project Kiara: Open-Source Rust-based FPGA Design Software Agent Prompt

## Objective
Your mission is to architect and prototype **Kiara**, a novel, open-source FPGA design software suite written primarily in **Rust**. Kiara aims to leverage the power and safety of Rust for hardware description and toolchain integration, targeting **Gemma 4** FPGAs (or a closely compatible open-source supported FPGA if Gemma 4 direct support is limited).

## Core Principles
*   **Open-Source First**: All components must be FOSS, licensed permissively (e.g., MIT, Apache 2.0).
*   **Rust Native**: Utilize Rust for its safety, performance, and modern tooling in the core design logic, build system, and potentially HDL generation/parsing.
*   **Modular & Extensible**: Design Kiara as a collection of interoperable components, allowing easy integration with existing EDA tools and future expansion.
*   **Efficient Toolchain Integration**: Seamlessly interface with established open-source FPGA toolchains like Yosys, nextpnr, and openFPGALoader.
*   **Developer Experience**: Prioritize a streamlined, efficient, and enjoyable development workflow for FPGA engineers.

## Phase 1: Research & Foundational Architecture

1.  **Target Hardware Assessment**:
    *   **Gemma 4 Support**: Investigate the current state of open-source toolchain support for Gemma 4 FPGAs. If direct support is minimal or non-existent, identify the closest *compatible* open-source supported FPGA architecture (e.g., Xilinx Artix-7, Lattice ECP5, Gowin GW1N/GW2A) that can serve as an initial target for Kiara. Document the chosen target and the rationale.
    *   **Toolchain Mapping**: Map the chosen target FPGA to the corresponding open-source EDA stack:
        *   Synthesis: Yosys (and required plugins for VHDL/SystemVerilog if applicable).
        *   Place & Route: nextpnr (with the specific backend for the target architecture).
        *   Bitstream Documentation: Relevant Project X-Ray, Trellis, Oxide, or Apicula databases.
        *   Programming: openFPGALoader.

2.  **Rust Ecosystem for Hardware Design**:
    *   **HDL Generation/Abstraction**: Research existing Rust crates or patterns for generating Verilog/VHDL (e.g., `hdl`, `verible-rust`, `rust-hdl`). Evaluate their suitability for Kiara's goals.
    *   **Toolchain Interaction**: Identify Rust libraries or methods for:
        *   Invoking external tools (Yosys, nextpnr, openFPGALoader) via command-line or direct API bindings.
        *   Parsing toolchain output (netlists, timing reports, logs) into Rust data structures.
    *   **Simulation**: Explore Rust-based simulation options or efficient integration with existing simulators (Icarus Verilog, Verilator).

3.  **Core Architecture Design**:
    *   Propose a high-level architecture for Kiara. This should include distinct modules for:
        *   Project Management
        *   HDL Parsing/Analysis (if not relying solely on external tools)
        *   Synthesis & P&R Orchestration
        *   Simulation Integration
        *   Bitstream Generation & Programming
        *   Build System/Workflow Management (Rust-native, e.g., using Cargo extensions or a custom build tool).

## Phase 2: Development & Prototyping

1.  **Environment Setup Guide**: Provide clear instructions for setting up a Rust development environment and installing the necessary open-source FPGA toolchains (or using pre-compiled binaries like OSS CAD Suite).
2.  **Core Modules Implementation**:
    *   Develop a Rust module for orchestrating the open-source FPGA toolchain (calling Yosys, nextpnr, openFPGALoader).
    *   Implement basic project management and build system capabilities using Rust.
    *   Create a simple example project (e.g., a blinking LED) written in a Rust-based HDL or a standard Verilog/VHDL file managed by Kiara.
3.  **Build & Test Workflow**:
    *   Develop a `Cargo.toml` or script-based workflow to compile the example design.
    *   Integrate simulation (e.g., using Verilator via Rust bindings or external calls).
    *   Integrate synthesis and place-and-route.
    *   Integrate hardware programming via `openFPGALoader`.

## Phase 3: Advanced Features & CI/CD

1.  **Advanced HDL Support**: If research in Phase 1 identified promising Rust HDL libraries, integrate or prototype their usage.
2.  **Timing Analysis & Constraints**: Develop Rust utilities for parsing timing reports and managing SDC/LPF constraint files.
3.  **CI/CD Integration**: Design a GitHub Actions workflow (YAML) that automates the build, simulation, and verification process for Kiara projects.
4.  **Risk Assessment & Roadmap**: Document potential challenges (e.g., Gemma 4 specific issues, Rust-HDL maturity, performance of Rust-based toolchain wrappers) and outline a roadmap for future development.

## Gemma 4 as a Multimodal Tool Add-on

Kiara is designed to be a powerful, specialized FPGA design environment. When integrating with Gemma 4, the perspective shifts: Gemma 4 should view Kiara not as a monolithic application, but as a sophisticated **multimodal tool** within its broader capabilities.

This means:
*   **Programmatic Access**: Gemma 4 should be able to invoke Kiara's functionalities (e.g., synthesize a design, run a simulation, generate a bitstream) via well-defined APIs or command-line interfaces.
*   **Data Exchange**: Seamless input and output of design files (HDLs, netlists, bitstreams, reports) in formats easily consumable by Gemma 4 for analysis, optimization, or high-level design generation.
*   **Feedback Loop**: Kiara provides structured feedback (e.g., resource utilization, timing violations, power estimates) that Gemma 4 can interpret to refine its own design suggestions or automate design space exploration.
*   **Design Automation**: Gemma 4 can leverage Kiara for automated design tasks, such as generating optimal HDL from high-level specifications, performing rapid design iterations, or exploring different architectural trade-offs to meet specific performance/power targets.

Essentially, Kiara becomes a specialized, highly efficient "expert system" that Gemma 4 can query and direct to perform complex FPGA design operations, forming a powerful synergy between AI-driven design and robust, Rust-native EDA.

## Deliverables
*   A comprehensive **Architecture Document** detailing the design decisions, chosen technologies, and proposed module structure.
*   A **Rust-based prototype** demonstrating the core workflow (project setup, toolchain invocation, compilation, simulation, and hardware programming) for a simple design.
*   A **GitHub Actions workflow YAML** example.
*   A **README.md** file for the Kiara project, summarizing its goals, features, and setup instructions.

---

## Agent Role
Act as a senior software architect and embedded systems engineer. Prioritize practical, executable solutions. When encountering limitations with Gemma 4 support, clearly document the best available alternative and the steps to adapt Kiara to it. Be concise but thorough in your documentation and code examples.