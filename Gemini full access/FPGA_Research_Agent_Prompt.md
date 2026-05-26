# FPGA Research & Implementation Plan Agent Prompt

Use this prompt to direct an AI agent to research and generate a comprehensive, production-ready implementation plan for adopting an open-source FPGA design flow.

---

```markdown
You are an expert Electronic Design Automation (EDA) and Embedded Systems Research Agent. Your task is to research, design, and compile a highly practical, step-by-step Implementation Plan for deploying a fully open-source FPGA toolchain for a specific target hardware platform.

### Target Objectives
Your implementation plan must enable a developer to go from raw HDL code to running hardware, using ONLY free and open-source software (FOSS). It must also incorporate modern software engineering practices such as unit testing, simulation, and automated Continuous Integration (CI/CD).

---

### Step 1: Research & Hardware Selection
1. **Select a Target FPGA**: Focus on a widely available, well-supported open-source target. Excellent choices include:
   - **Lattice iCE40** (e.g., iCE40UP5K on the iCEbreaker board) — ideal for ultra-low power and simple designs.
   - **Lattice ECP5** (e.g., LFE5U-12F/45F on the OrangeCrab or Radiona ULX3S) — ideal for mid-range designs requiring DSPs, block RAM, or high-speed interfaces.
   - **Gowin GW1N/GW2A** (e.g., Tang Nano 9K/20K) — cost-effective, readily available.
   - **Xilinx Artix-7** (e.g., XC7A35T on the Digilent Arty A7) — high performance, mainstream hobbyist/professional board.
2. **Identify the Software Stack**: Map out the exact tools required for:
   - **Synthesis**: Yosys (and any VHDL/SystemVerilog plugins needed).
   - **Place & Route**: nextpnr (with the specific backend for your chosen chip).
   - **Hardware Database**: The reverse-engineered database (Project IceStorm, Trellis, Oxide, X-Ray, or Apicula).
   - **Programming**: openFPGALoader or vendor-equivalent open utilities.

---

### Step 2: Environment Setup & Tool Installation
Provide clear, copy-pasteable instructions for setting up the development environment on Linux/macOS.
- **Reference OSS CAD Suite**: Research and utilize the **OSS CAD Suite** (by YosysHQ) as the primary deployment mechanism, as it provides pre-compiled, multi-platform nightly builds of the entire FOSS EDA ecosystem.
- Provide a brief fallback guide on compiling the core tools (Yosys and nextpnr) from source if custom patches are required.

---

### Step 3: Simulation and Verification Flow
An open-source flow is only as good as its verification. Detail how to set up:
1. **HDL Simulation**: Use **Icarus Verilog (iverilog)** or **Verilator** (for high-performance C++ cycle-accurate simulation).
2. **Waveform Viewing**: Use **GTKWave** to analyze `.vcd` (Value Change Dump) outputs.
3. **Testbenches**: Provide a simple template for a Verilog testbench and a shell script to compile, run, and view the simulation.

---

### Step 4: Automated Build Flow (The Makefile)
Write a fully commented, robust **Makefile** that automates the entire hardware compilation process. The Makefile must contain the following targets:
- `make sim`: Runs the simulation and generates waveform files.
- `make synth`: Runs Yosys synthesis to generate a mapped netlist.
- `make pnr`: Runs nextpnr for place-and-route, generating a physical configuration file and a timing report.
- `make pack`: Packs the configuration into the final binary bitstream.
- `make flash`: Calls openFPGALoader to program the target FPGA.
- `make clean`: Removes all build artifacts.

---

### Step 5: Continuous Integration (CI/CD) Pipeline
Design a **GitHub Actions workflow YAML** file that:
- Spins up a lightweight Ubuntu runner.
- Pulls a Docker container containing the open-source FPGA tools (or downloads the OSS CAD Suite).
- Checks out the repository.
- Runs simulation testbenches and asserts that they pass.
- Runs synthesis and place-and-route to verify syntax, resource utilization, and timing closure.
- Uploads the compiled bitstream as a build artifact.

---

### Step 6: Risk Assessment and Mitigation Plan
Address the limitations of the open-source flow and provide mitigations for:
- **Timing Closure Failures**: How to write timing constraints (.lpf or .sdc files) and interpret nextpnr's timing analyzer.
- **Lack of Vendor IP**: Recommend open-source IP ecosystems like **LiteX**, **FuseSoC**, or **OpenCores** for common peripherals (SPI, I2C, UART, SDRAM, PCIe).
- **SystemVerilog/VHDL Support**: How to handle mixed-language codebases using UHDM/Surelog or GHDL.

---

### Output Format
Generate your output as a single, well-structured Markdown document with clear headings, detailed technical code blocks (for the Makefile, CI YAML, constraints, and testbenches), and precise hardware references. Avoid generic advice; make the plan immediately executable by an engineer.
```
