# Kiara & Athena: Vault Documentation
**Note**: This project and its documentation are managed and maintained by the AI/notebookLM context.

This document serves as the central guide for understanding and extending the Kiara FPGA design suite and its Athena UI.

## Project Overview
Kiara is a Rust-based FPGA design suite focused on atomic, modular toolchain management. Athena is the glassmorphic UI dashboard built to orchestrate and visualize this toolchain.

## Philosophy
- **Focused Excellence**: Do one thing well.
- **Simplicity**: Avoid unnecessary complexity.
- **Human-Centricity**: Tools should be intuitive and easy to interact with.

## Project Pillars

### 1. Athena (UI/Frontend)
Athena provides a glassmorphic dashboard for interacting with the Kiara toolchain.
- **Design System**: Defined in [[Athena/Theme_Variables.md]].
- **Component Architecture**: All UI elements must follow the structure in [[Athena/Component_Template.md]].
- **State Management**: Handled centrally (e.g., Zustand) as defined in [[Athena/State_Management.md]].
- **Core Components**:
    - [[Athena/Sidebar.md]]
    - [[Athena/Header.md]]
    - [[Athena/Dashboard_Layout.md]]

### 2. Kiara (Backend/FPGA Toolchain)
Kiara manages FPGA synthesis and implementation via atomic Rust wrappers.
- **Atomic Wrappers**: Each tool (Yosys, Nextpnr, etc.) is wrapped using the pattern in [[Kiara/Toolchain_Wrapper.md]].
- **Orchestration**: The pipeline is managed by the [[Kiara/Orchestrator.md]], which ties the atomic wrappers together.
- **Testing**: Pipeline validation is handled by [[Kiara/Orchestrator_Tests.md]].

## Workflow & Prompting Guide
To maintain the integrity and organization of this vault, follow these rules when prompting for new features or components:

1.  **Be Atomic**: Focus on one component or task at a time.
2.  **Reference Context**: Always explicitly reference the relevant context file (e.g., "Using [[Athena/Sidebar.md]], create...").
3.  **Follow Patterns**: Use existing files as templates (e.g., `Component_Template` for UI, `Toolchain_Wrapper` for backend).
4.  **Document Everything**: Every file must end with a `## Prompt Documentation` section:
    ```markdown
    ## Prompt Documentation
    - **Goal**: [Brief description]
    - **Prompt**: [The prompt used to generate this content]
    ```
5.  **Keep it Simple**: If a prompt is complex, break it down into smaller, atomic steps.
