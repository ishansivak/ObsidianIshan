# Erebor Studios: Open-Source EDA & Sovereign Applications
**Note**: This project and its documentation are managed with uncompromising focus and precision.

Erebor Studios is dedicated to building foundational, high-performance tools for decentralized compute, hardware-as-code engineering, and sovereign consumer applications.

---

## The Erebor Philosophy
*   **Focused Excellence**: Do a few things, but make them insanely great.
*   **Simplicity**: Strip away the noise and complexity until only what is pure and functional remains.
*   **Human-Centricity**: Technology is an extension of the human mind. Tools must be beautiful, intuitive, and empowering.

---

## The Whiteboard Grid: Our Four Pillars

```
                  ┌───────────────────────┬───────────────────────┐
                  │       PRO ENGINE      │    CONSUMER PRODUCT   │
 ┌────────────────┼───────────────────────┼───────────────────────┤
 │                │       [[Kiara]]       │       [[Bhejo]]       │
 │   SOFTWARE     │  Rust FPGA Synthesis  │  File-Sharing & UPI   │
 │                │  & Open-Source EDA    │  Decentralized App    │
 ├────────────────┼───────────────────────┼───────────────────────┤
 │                │      [[Athena]]       │     [[blackbox]]      │
 │   INTERFACE    │  Glassmorphic UI &    │  Saito, Mycelial, &   │
 │   & CORE IP    │   Dashboard Layout    │  Agri-Tech Research   │
 └────────────────┴───────────────────────┴───────────────────────┘
```

### 1. [[Kiara]] (The Pro Engine)
A Rust-native, AI-invocable FPGA design suite built to orchestrate open-source EDA tools (Yosys, nextpnr, openFPGALoader).
*   **Atomic Wrappers**: Each tool is wrapped using the pattern in [[Toolchain_Wrapper]].
*   **Orchestration**: Managed dynamically by the [[Orchestrator]], which compiles the project DAG.
*   **Testing**: Validated via the unit test suite in [[Orchestrator_Tests]].

### 2. [[Athena]] (The Pro Interface)
A stunning, glassmorphic UI dashboard built with Leptos 0.5 and Tailwind CSS to orchestrate and visualize the Kiara toolchain.
*   **Design System**: Defined in [[Theme_Variables]].
*   **Component Template**: All UI containers follow [[Component_Template]].
*   **State Management**: Handled centrally in [[State_Management]].
*   **Core Components**: [[Sidebar]], [[Header]], and [[Dashboard_Layout]].

### 3. [[Bhejo]] (The Consumer Application)
A revolutionary, decentralized peer-to-peer file-sharing application running on Saito and India's UPI payment rail, enabling secure, atomic, and instant content monetization and distribution.

### 4. [[blackbox/blackbox|blackbox]] (The Core IP / R&D)
Our private, high-value conceptual repository:
*   **[[Saito_Consensus|Saito Consensus]]**: Routing work mechanics, the burn fee, and the Golden Ticket lottery.
*   **[[UPI_Saito_Settlement|UPI-Saito Settlement]]**: Real-time atomic clearing bridging India's UPI with Saito.
*   **[[Mycelial_Network_Routing|Mycelial Network Routing]]**: Biological decentralized networks and fungal computing.
*   **[[AgriTech_Mesh_Network|Agri-Tech Mesh Network]]**: Off-grid LoRa mesh networks and Edge AI sensor nodes.

---

## Operational Protocol & Hygiene
To maintain the integrity and beauty of this vault, follow these rules:
1.  **Keep it Flat**: No redundant nesting or double-nested directories. Clean paths are mandatory.
2.  **No Redundancy**: Maintain a single source of truth. Delete duplicate summaries or notes immediately.
3.  **Be Atomic**: Focus on one component or task at a time. Refer to existing design patterns.
4.  **Absolute Precision**: Avoid LLM-generation junk, redundant placeholders, or "Prompt Documentation" blocks. Keep files clean, technical, and beautifully structured.

## 6. ANTI-HALLUCINATION & MACRO PROGRESS DIRECTIVE (MANDATORY)
1. **NO FAKE WORK**: The AI MUST NEVER hallucinate file modifications, reads, or deletions. You must verifiably execute tool calls for every single action you claim to perform.
2. **MACRO PROGRESS BAR**: The mandatory progress bar at the start of responses MUST reflect the **MACRO** progress of the actual engineering project (e.g., Kiara is at ~15%, Bhejo is ~10%). It MUST NEVER reflect the mere completion of the user's immediate chat prompt. NEVER output a 100% progress bar unless a massive, multi-week engineering milestone is genuinely shipped and verifiable in the codebase. We are nowhere near 100% done.