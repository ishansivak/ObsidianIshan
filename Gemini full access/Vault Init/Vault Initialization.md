---
tags:
  - init
  - pep-talk-compliant
---
# Vault Initialization

This folder contains the core files required for an agent to boot up and understand the vault's governance, structure, and current state.

## Core Directives
1. **Exhaustive Review**: Before any structural change, the agent MUST perform an exhaustive read-through of every single note in the vault.
2. **Connectivity Audit**: Every note must have at least two incoming and two outgoing links to relevant indices or related research notes. No orphans. No dead ends.
3. **Redundancy Purge**: Consolidate duplicates into a single "source of truth".
4. **Context Enrichment**: All notes must contain a minimum of 200 words of substantive, technical context. **ABJECT FAILURE TOLERANCE IS ZERO: Empty or sub-200-word notes are prohibited.**
5. **Graph Density**: Cross-domain links between `Biological Networks`, `Protocol Engineering`, and `Hardware Infrastructure` must be explicitly added.
6. **Graph Readability**: The graph view must be prioritized as an aesthetic, elegant, and navigable interface. Avoid excessive clustering; maintain clear, logical clusters around core indices.
7. **Data Respect**: Never overwrite or delete information without first verifying it is truly redundant.
8. **Cleanup Protocol**: All deleted/discarded materials MUST be moved to `[[tempTrash]]` instead of being permanently deleted.
9. **Historical Context**: Agents must read all past agent conversations in the `gemini-scribe/` subfolder before initiating any structural changes.
10. **Traversal Logic**: If you are confused by the structure, traverse the links to understand the notes. If the graph still appears cluttered or illogical, change the structure immediately to improve clarity. "Be like water."
11. **Measure Twice, Cut Once**: Always read the entire vault content before cutting. If you cut, be ruthless but precise.
12. **Visualization & Tooling**: Actively create and maintain Kanban boards and other visualization tools where useful.
13. **Logging**: Every maintenance run MUST log its changes and the reasoning behind those changes.

## Mandatory Mantras for Initialization
Every maintenance or startup loop MUST follow this mantra:
**Pep. Init. Read. Google. Clean up. Repeat.**

- **Pep**: Internalize the mission from [[Agent Pep Talk]].
- **Init**: Review these initialization protocols.
- **Read**: Scan every note for substantive content (200+ words).
- **Google**: Validate technical claims.
- **Clean up**: Excise, merge, and refine; move discarded items to [[tempTrash]].
- **Repeat**: Ensure no dead links or empty notes remain.
