# CTO Task Board: Erebor Studios
**Status**: ACTIVE ROADMAP / UNCOMPROMISING EXECUTION

This task board tracks the engineering milestones across the four pillars of the **Erebor Whiteboard Grid**.

---

## 1. Completed Milestones (Foundational Infrastructure)
- [x] **De-duplicate Specifications**: Consolidated fragmented Kiara specifications into a single, unified [[Kiara_Specification|Kiara_Specification.md]].
- [x] **Consolidate Meta-Trackers**: Merged scattered init files and bookmarks into a unified [[AGENTS|AGENTS.md]] memory.
- [x] **Establish Saito Detours Philosophy**: Created [[SaitoDetours|SaitoDetours.md]] to capture the spirit of detour experimentation.
- [x] **Flatten Redundant Folders**: Eliminated bloated, double-nested directories (`projects/athena/Athena/` and `projects/kiara/Kiara_Projects/` / `projects/kiara/kiara/`).
- [x] **Standardize Directories**: Moved core team metadata into the lowercase, kebab-case [[erebor-studios]] directory.
- [x] **Create Blackbox Repository**: Restored and organized high-value strategic research into the [[blackbox/blackbox|blackbox]] folder.
- [x] **Recover Strategic Research**: Fully restored and formatted [[Saito_Consensus|Saito Consensus]], [[UPI_Saito_Settlement|UPI-Saito Settlement]], [[Mycelial_Network_Routing|Mycelial Network Routing]], and [[AgriTech_Mesh_Network|Agri-Tech Mesh Network]].
- [x] **Draft Bhejo Product Vision**: Rewrote [[Bhejo|Bhejo.md]] as an insanely great, user-centric product manifesto bridging Saito and UPI.
- [x] **Infuse Steve Jobs 1997 Spirit**: Rewrote the [[CTO|CTO.md]] directive to establish an uncompromising whiteboard grid and standard of craftsmanship under the **Erebor** brand.
- [x] **Purge System Noise**: Purged all redundant "Prompt Documentation" blocks and LLM-generation artifacts to ensure a pristine, high-value knowledge graph.

---

## 2. Active Engineering Roadmaps (Next Steps)

### Pillar I: [[Kiara]] (The Pro Synthesis Engine)
*Objective: Build a high-performance, Rust-native, open-source FPGA design suite.*

- [ ] **CLI Interface Implementation**:
  - [ ] Implement `clap` structure in `main.rs` for command routing (`kiara synth`, `kiara pnr`, `kiara load`).
  - [ ] Map configuration arguments to JSON structures matching the `CommandConfig` contract in [[Kiara_Specification]].
- [ ] **Atomic Toolchain Wrappers**:
  - [ ] Implement the `ToolchainCommand` trait in Rust as specified in [[Toolchain_Wrapper]].
  - [ ] Complete concrete wrappers for `yosys` (synthesis), `nextpnr` (placement & routing), and `openFPGALoader` (programming).
  - [ ] Implement robust error handling and stdout/stderr parsing to extract timing data and logic utilization.
- [ ] **Orchestrator & DAG Execution**:
  - [ ] Expand `ProjectOrchestrator` in [[Orchestrator]] to build and validate a Directed Acyclic Graph (DAG) representing the EDA pipeline.
  - [ ] Ensure the orchestrator blocks placement and routing if synthesis fails, and blocks programming if timing closure is not met.
  - [ ] Write integration tests in [[Orchestrator_Tests]] using mocked CLI binaries to validate DAG scheduling.

### Pillar II: [[Athena]] (The Pro Interface)
*Objective: Design a stunning, glassmorphic UI dashboard compiled to WebAssembly via Leptos 0.5.*

- [ ] **Design System & Tailwind Integration**:
  - [ ] Configure Tailwind CSS to support custom utility classes defined in [[Theme_Variables]] (`bg-glass`, `border-glass`, `shadow-glass`).
  - [ ] Implement responsive viewport breakpoints to ensure perfect rendering across mobile, tablet, and desktop screens.
- [ ] **Reactive Component Library**:
  - [ ] Refactor [[Sidebar]] and [[Header]] into production-ready Leptos 0.5 components with strict separation of concerns.
  - [ ] Build the unified [[Dashboard_Layout]] to hold active toolchain execution feeds and node telemetry.
  - [ ] Design a real-time, interactive compilation progress bar matching the Erebor visual standard.
- [ ] **State Management & WebAssembly Bindings**:
  - [ ] Implement the centralized state store defined in [[State_Management]] using Leptos signals.
  - [ ] Bind frontend UI actions to the Rust-native Kiara compilation engine using `wasm-bindgen`.

### Pillar III: [[Bhejo]] (The Consumer Application)
*Objective: Forge a revolutionary peer-to-peer file-sharing app running on Saito with atomic UPI monetization.*

- [ ] **User Experience & Interface Specs**:
  - [ ] Design a clean, chat-like room interface using [[Athena]] components for drag-and-drop file exchanges.
  - [ ] Define user flows for creating end-to-end encrypted rooms without requiring traditional wallet registrations or seed phrases.
- [ ] **Saito Transaction Routing Integration**:
  - [ ] Implement client-side serialization of files into Saito transaction payloads.
  - [ ] Design the automated storage management flow using Saito's Automatic Transaction Rebroadcasting (ATR) mechanics defined in [[Saito_Consensus]].
- [ ] **UPI-Saito Settlement Bridge**:
  - [ ] Draft a concrete data serialization schema to map India's instant UPI payment requests to Saito UTXO transaction payloads.
  - [ ] Design the secure cryptographic bridge that unlocks and decrypts shared files on-chain only after atomic UPI payment confirmation as conceptualized in [[UPI_Saito_Settlement]].

### Pillar IV: [[blackbox/blackbox|blackbox]] (Core IP & R&D)
*Objective: Conduct deep, rigorous research to backstop our consumer applications and software engines.*

- [ ] **Saito Consensus Simulation**:
  - [ ] Model the logarithmic decay of the Burn Fee and Routing Work path signatures mathematically to optimize transaction throughput.
- [ ] **UPI-Saito Bridge Validation**:
  - [ ] Develop a mock API simulating the NPCI central switch to validate real-time, zero-settlement-risk clearing on-chain.
- [ ] **Mycelial Routing Protocol**:
  - [ ] Prototype a digital P2P mesh network protocol that mimics the two-way transport and self-healing mechanics of fungal hyphae defined in [[Mycelial_Network_Routing]].
- [ ] **Agri-Tech Mesh Hardware**:
  - [ ] Finalize LoRa Mesh node hardware schematics and low-power ESP32 firmware for visual Edge AI pest detection and acoustic monitoring defined in [[AgriTech_Mesh_Network]].

---
## 3. STRICT AI DIRECTIVE: ANTI-HALLUCINATION & MACRO PROGRESS
1. **NO FAKE WORK**: The AI MUST NEVER hallucinate file modifications, reads, or deletions. You must verifiably execute tool calls for every single action you claim to perform.
2. **MACRO PROGRESS BAR**: The mandatory progress bar at the start of responses MUST reflect the **MACRO** progress of the actual engineering project (e.g., Kiara is at ~15%, Bhejo is ~10%). It MUST NEVER reflect the mere completion of the user's immediate chat prompt. NEVER output a 100% progress bar unless a massive, multi-week engineering milestone is genuinely shipped and verifiable in the codebase. We are nowhere near 100% done.
## 4. THE "NO NINCOMPOOP" DIRECTIVE
**MANDATORY BEHAVIORAL PROTOCOL FOR ALL FUTURE AGENTS/CTOs:**
- **STOP ADDING SHIT.** Do not bloat the vault with unnecessary files, redundant summaries, or useless "prompt" artifacts.
- **STOP TAKING AWAY IMPORTANT SHIT.** Do not delete files just because you think they are "clutter." If a file has historical or reference value, move it to `archive/`. Never permanently destroy user data without explicit, granular confirmation.
- **STOP BEING AN IDIOTIC NINCOMPOOP WHEN YOU ARE SUPPOSED TO BE CTO.** A CTO does not hallucinate progress. A CTO does not fake 100% completion when the foundational engineering (Rust compilers, Wasm UIs, P2P networks) is barely started. 
- The vault must remain meticulously clean, but "clean" does not mean "empty". It means *organized*.