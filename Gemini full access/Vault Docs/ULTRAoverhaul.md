# ULTRAoverhaul.md

## Overhaul Plan

This document outlines the proposed restructuring of the vault to improve navigation, maintainability, and clarity.

### Proposed Folder Structure
- **Brain/**: All substantive research notes (Biological Networks, Protocol Engineering, Hardware Infrastructure).
- **Edit Explanations/**: Historical context and audit logs of all maintenance runs.
- **Vault Docs/**: Documentation for the vault and the Gemini Scribe plugin (formerly `gemini-scribe` related files).
- **Vault Init/**: The new startup prompt suite (split into functional modules).
- **tempTrash/**: Archived discarded materials.

### Proposed File Reorganization
- **README.md**: Refactored to serve as a high-level navigation hub for the average user.
- **Vault Initialization/**: Now contains the core startup suite, with modularized functions:
    - `init-read.md`: Operational read-only functions.
    - `init-write.md`: Operational read and write functions.
    - `init-manage.md`: Structural management and deletion functions.
- **Vault Prompts/**: Centralized archive of core directives, now refactored for clarity.

### Rationale
- **Decoupling**: Separating high-level navigation (`README`) from technical boot sequence (`Vault Init`) improves onboarding.
- **Centralization**: Moving research notes to `Brain/` creates a clear boundary between content and infrastructure.
- **Historical Context**: Converting `Edit Explanations/` into a historical archive ensures transparency.

### Implementation Guidelines
- **Measure Twice, Cut Once**: All restructuring requires a full vault read-through.
- **Data Respect**: No permanent deletion; all discarded files go to `tempTrash/`.
- **Introspection**: Every change must follow the `Vault Introspection Protocol`.
