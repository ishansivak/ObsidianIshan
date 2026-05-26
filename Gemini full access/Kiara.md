# Project Kiara: Open-Source Rust-based FPGA Design Software Agent Prompt

## Objective
Your mission is to architect and prototype **Kiara**, a novel, open-source FPGA design software suite written primarily in **Rust**. Kiara aims to leverage the power and safety of Rust for hardware description and toolchain integration, targeting open-source-friendly FPGA architectures.

## Core Principles
*   **Open-Source First**: All components must be FOSS, licensed permissively (e.g., MIT, Apache 2.0).
*   **Rust Native**: Utilize Rust for its safety, performance, and modern tooling in the core design logic, build system, and potentially HDL generation/parsing.
*   **Modular & Extensible**: Design Kiara as a collection of interoperable components, allowing easy integration with existing EDA tools and future expansion.
*   **Efficient Toolchain Integration**: Seamlessly interface with established open-source FPGA toolchains like Yosys, nextpnr, and openFPGALoader.
*   **Developer Experience**: Prioritize a streamlined, efficient, and enjoyable development workflow for FPGA engineers.

## Phase 1: Research & Foundational Architecture

1.  **Target Hardware Assessment**:
    *   **Initial Target Selection**: Kiara is designed to leverage open-source EDA toolchains. The project will target mature, FOSS-supported FPGA architectures.
    *   **Recommended Alternatives**:
        *   **Lattice iCE40**: The gold standard for open-source FPGA development. Extremely well-supported by Yosys and nextpnr-ice40. Ideal for initial prototyping and simple designs (e.g., blinking LED).
        *   **Lattice ECP5**: Higher capability than iCE40, excellent open-source support via Project Trellis. Suitable for complex designs and soft-core processors.
        *   **Xilinx Artix-7**: Widely used in industry. Supported via Project X-Ray. Good for bridging the gap between FOSS prototyping and industry-standard hardware.
    *   **Toolchain Mapping**: Map the chosen target (e.g., iCE40 or ECP5) to the corresponding open-source EDA stack:
        *   Synthesis: Yosys (and required plugins for VHDL/SystemVerilog if applicable).
        *   Place & Route: nextpnr (with the specific backend for the target architecture).
        *   Bitstream Documentation: Relevant Project X-Ray, Trellis, or Apicula databases.
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
        *   HDL Parsing/Analysis
        *   Synthesis & P&R Orchestration
        *   Simulation Integration
        *   Bitstream Generation & Programming
        *   Build System/Workflow Management (Rust-native, e.g., using Cargo extensions).

## Phase 2: Development & Prototyping

1.  **Environment Setup Guide**: Provide clear instructions for setting up a Rust development environment and installing the necessary open-source FPGA toolchains (or using pre-compiled binaries like OSS CAD Suite).
2.  **Core Modules Implementation**:
    *   Develop a Rust module for orchestrating the open-source FPGA toolchain.
    *   Implement basic project management and build system capabilities using Rust.
    *   Create a simple example project (e.g., a blinking LED) managed by Kiara.
3.  **Build & Test Workflow**:
    *   Develop a `Cargo.toml` or script-based workflow to compile the example design.
    *   Integrate simulation (e.g., using Verilator).
    *   Integrate synthesis, place-and-route, and hardware programming.

## Phase 3: Advanced Features & CI/CD

1.  **Advanced HDL Support**: Integrate or prototype promising Rust HDL libraries.
2.  **Timing Analysis & Constraints**: Develop Rust utilities for parsing timing reports and managing SDC/LPF constraint files.
3.  **CI/CD Integration**: Design a GitHub Actions workflow (YAML) that automates the build, simulation, and verification process.
4.  **Risk Assessment & Roadmap**: Document potential challenges (e.g., Rust-HDL maturity, performance of Rust-based toolchain wrappers) and outline a roadmap.

## AI Integration (e.g., Gemma/Gemini)

Kiara is designed to be a powerful, specialized FPGA design environment. When integrating with LLMs like Gemma, the perspective shifts: the LLM should view Kiara not as a monolithic application, but as a sophisticated **multimodal tool** within its broader capabilities.

This means:
*   **Programmatic Access**: The LLM should be able to invoke Kiara's functionalities (e.g., synthesize a design, run a simulation, generate a bitstream) via well-defined APIs or command-line interfaces.
*   **Data Exchange**: Seamless input and output of design files (HDLs, netlists, bitstreams, reports) in formats easily consumable by the LLM for analysis, optimization, or high-level design generation.
*   **Feedback Loop**: Kiara provides structured feedback (e.g., resource utilization, timing violations, power estimates) that the LLM can interpret to refine its own design suggestions or automate design space exploration.
*   **Design Automation**: The LLM can leverage Kiara for automated design tasks, such as generating optimal HDL from high-level specifications, performing rapid design iterations, or exploring different architectural trade-offs.

Essentially, Kiara becomes a specialized, highly efficient "expert system" that the LLM can query and direct to perform complex FPGA design operations, forming a powerful synergy between AI-driven design and robust, Rust-native EDA.

## Deliverables
*   A comprehensive **Architecture Document** detailing the design decisions, chosen technologies, and proposed module structure.
*   A **Rust-based prototype** demonstrating the core workflow.
*   A **GitHub Actions workflow YAML** example.
*   A **README.md** file for the Kiara project.

---

## Agent Role
Act as a senior software architect and embedded systems engineer. Prioritize practical, executable solutions. When encountering limitations with target hardware support, clearly document the best available alternative and the steps to adapt Kiara to it. Be concise but thorough in your documentation and code examples.
