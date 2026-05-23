# Vault Initialization (Vault Init)

This folder contains the core files required for an agent to boot up and understand the vault's governance, structure, and current state.

## Core Directives
1. **Exhaustive Review**: Before any structural change, the agent MUST perform an exhaustive read-through of every single note in the vault.
2. **Connectivity Audit**: Every note must have at least two incoming and two outgoing links to relevant indices or related research notes. No orphans. No dead ends.
3. **Redundancy Purge**: Consolidate duplicates into a single "source of truth".
4. **Context Enrichment**: All notes must contain a minimum of 200 words of substantive, technical context.
5. **Graph Density**: Cross-domain links between `Biological Networks`, `Protocol Engineering`, and `Hardware Infrastructure` must be explicitly added.
6. **Graph Readability**: The graph view must be prioritized as an aesthetic, elegant, and navigable interface. Avoid excessive clustering; maintain clear, logical clusters around core indices.
7. **Data Respect**: Never overwrite or delete information without first verifying it is truly redundant. 
8. **Cleanup Protocol**: All deleted/discarded materials MUST be moved to `[[tempTrash]]` instead of being permanently deleted.
9. **Historical Context**: Agents must read all past agent conversations in the `gemini-scribe/` subfolder before initiating any structural changes.
10. **Traversal Logic**: If you are confused by the structure, traverse the links to understand the notes. If the graph still appears cluttered or illogical, change the structure immediately to improve clarity. "Be like water."

## Files
- [[README]]
- [[Vault Prompts]]
- [[Vault Cleanup Plan]]
- [[Specific Vault Prompt]]
- [[Research Index]]
- [[Biological Index]]
- [[Protocol Index]]
- [[Infrastructure Index]]
- [[Vault Introspection Protocol]]
