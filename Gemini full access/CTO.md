---
Progress: 55%
---
# CTO Directive: Vault Clean-up & Reorganization Plan

This document serves as the strategic blueprint for cleaning up the Soma Studios vault to establish a pristine, modular foundation. This reorganization is crucial to prepare our workspace for the upcoming **new app** while maintaining the integrity of our existing pillars: **Kiara**, **Athena**, and the **Saito Detours** philosophy.

---

## 1. Ishan's Directive (2026-05-28)
> *"hi, so you are my CTO. we are trying to build a new app for our studio. I will describe it to you soon. But right now priority number one is to read init and everything else and then make a CTO.md that describes all the core concepts needed to clean up this vault. Also make sure you capture this message and all necessary context about the vault."*

---

## 2. Current Vault Assessment & Context
Our vault currently holds three primary pillars governed by the **Soma Studios** philosophy:

### A. Soma Studios (The Umbrella)
*   **Philosophy**: Focused Excellence, Simplicity, Craftsmanship, Human-Centricity, and Seamless Integration (defined in [[Soma Studios/Philosophy.md|Philosophy.md]]).
*   **Vision**: Build foundational tools for decentralized compute and hardware-as-code engineering with absolute transparency and rigorous standards (defined in [[Soma Studios/Portfolio.md|Portfolio.md]]).
*   **Master Prompt**: Governs operations, tracking progress via percentages, using modular Rust, JSON interfaces, and Leptos components (defined in [[Soma Studios/Master_Prompt.md|Master_Prompt.md]]).
*   **Spirit**: Captured in [[SaitoDetours.md|SaitoDetours.md]], representing our commitment to exploration, refinement, and craftsmanship.

### B. Kiara (The FPGA Design Suite)
*   **Objective**: A Rust-native, AI-invocable FPGA design suite orchestrating open-source EDA tools (Yosys, nextpnr, openFPGALoader).
*   **Current State**:
    *   Core specs and Rust files are now consolidated in `kiara/Kiara_Specification.md`.
    *   Key architectural patterns use atomic wrappers implementing a `ToolchainCommand` trait and a DAG-based `ProjectOrchestrator`.

### C. Athena (The UI Dashboard)
*   **Objective**: A glassmorphic UI dashboard built to orchestrate and visualize the toolchain, emphasizing trustless transparency in decentralized systems.
*   **Current State**:
    *   Housed in the `Athena/` directory.
    *   Built using Leptos 0.5 with Tailwind CSS.
    *   Core components like Sidebar, Header, Dashboard Layout, and State Management are fully defined.

---

## 3. Core Clean-up Concepts & Reorganization Plan
To make room for the new app and prevent cross-project clutter, we must enforce strict workspace hygiene and modularity.

### Concept 1: Elimination of Redundancy (De-duplication)
*   **Action**: `notebookLM.md` merged into `README.md` and deleted. - *COMPLETE*.
*   **Action**: Consolidated `Kiara_Master_Spec.md` and `Kiara_Core_Spec.md` into `Kiara_Specification.md`. - *COMPLETE*.
*   **Action**: Move `archive/Final prompt.md` (which is an unrelated LLM frontend) out of active project space or keep it strictly archived.

### Concept 2: Directory Streamlining (Project Isolation)
Currently, Kiara files are scattered across `kiara/` and `Kiara_Projects/`. We will establish a clean, nested project hierarchy to isolate Kiara, Athena, and the upcoming new app:
```
/
├── README.md (Single source of truth for the vault)
├── AGENTS.md (Unified AI memory and context)
├── CTO.md (This strategic blueprint)
├── SaitoDetours.md (Spirit of the channel)
├── soma-studios/
│   ├── Philosophy.md
│   ├── Portfolio.md
│   └── Master_Prompt.md
└── projects/
    ├── kiara/ (Consolidated Rust codebase, specs, and research)
    ├── athena/ (All Leptos UI components and design specs)
    └── [new-app]/ (Reserved clean directory for our upcoming studio application)
```

### Concept 3: Meta-File Consolidation
*   **Current State**: `init.md` and `bookmark.md` consolidated into `AGENTS.md`. - *COMPLETE*.

---

## 4. Reorganization Roadmap (Phased)
*   **Phase 1 (20%)**: Initial assessment and strategic plan formulation (**CTO.md** created) - *COMPLETE*.
*   **Phase 2 (50%)**: File consolidation (De-duplicate specs, merge `notebookLM` and `README`, streamline meta-trackers) - *COMPLETE*.
*   **Phase 3 (80%)**: Directory restructuring (Create `projects/` directory, migrate Kiara and Athena files, resolve all wiki-links).
*   **Phase 4 (100%)**: Verification and preparation of `projects/[new-app]/` for Ishan's product requirements.

---

### Progress Tracking
- **Current Progress**: 55% (Strategic assessment, context capture, Phase 2 file consolidation, and SaitoDetours created).
