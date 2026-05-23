# Vault Prompts

This file stores the core directives and prompts used to maintain the vault's structure and integrity.

## Primary Directives
1. "Do more. More context, more structure. Less redundant crap. More logic. Better links. Less dead links. More navigable in the graph view. Think for a long time before taking actions, but take lots of action."
2. "You didn't make the necessary adjustments yet. do more. more context more structure. less redundant crap. more logic. better links. less dead links. more navigable in the graph view. go go go. think for a long time before taking actions. but take lots of action. also make sure that the readme keeps in store a bunch of context about the vault, including this exact prompt for future use. Append more strict alternatives to this prompt in the readme so users can make sure you actually go back and read every. single. note. now get started. dont be lazy. take your time. think. then act ruthlessly and cunningly. be like water. let the concepts and structures flow naturally as opposed to artificial knowledge structures."
3. "When modifying this vault, you must act with extreme respect for existing data. Before adding, deleting, or restructuring, you must read all affected notes to understand their nuance. Do not delete or add stuff before thinking long and hard. Act ruthlessly against redundancy, but with surgical precision to preserve the underlying intellectual capital."
4. **Traversal Logic**: If you are confused by the structure, traverse the links to understand the notes. If the graph still appears cluttered or illogical, change the structure immediately to improve clarity. "Be like water."

## Operational Guidelines
- Agents MUST act with extreme respect for existing data.
- Always perform an exhaustive review before modifying structure.
- Maintain high graph density through bidirectional links and cross-domain references.
- Documentation must be substantive (minimum 200 words per note).

## Specific Vault Prompt
For detailed operational guidance, refer to [[Specific Vault Prompt]]. This prompt synthesizes all domain knowledge (mycology, protocols, hardware) to ensure high-density, logical linking. Agents must adopt this persona when modifying the vault.

## Introspection Protocol
Before finishing any maintenance run, agents must perform the self-check documented in [[Vault Introspection Protocol]]. If any check fails, the maintenance must continue until compliance is achieved.
## Maintenance Instructions
- Agents must reference `[[Vault Initialization]]` as the primary boot sequence.
- **Measure Twice, Cut Once**: All changes must be verified against existing context.
- **Trash Handling**: Any deleted or discarded note content must be moved to `[[tempTrash]]`. No permanent deletion without verification.

## Graph Aesthetics and Readability
To ensure the graph view is beautiful, elegant, and navigable:
- Maintain **clear hierarchy**: Research Notes -> Category Indices -> Research Index.
- Avoid "super-nodes" that connect to everything; distribute links logically.
- Use consistent cross-linking between domains (`Biological`, `Protocol`, `Infrastructure`) to create a balanced, symmetric graph.
- Keep the graph clear of "orphans" and "dead ends" by ensuring every note has a path back to an index.

### Maintenance Logging
Every maintenance run must be documented in a new file within `Edit Explanations/`. The agent must explicitly state:
1. What was changed (notes, structures, links).
2. The specific reasoning for those changes, referencing the `Vault Cleanup Plan` and `Vault Introspection Protocol`.
3. Any visualization tools (like Kanban boards) created or updated during the run.

### Visualization Mandate
If the graph view is cluttered, the agent must create Kanban boards or other visualization tools to offload complexity from the core graph.

## Graph Aesthetic & Readability Protocol (Added 2026-05-23)
- **Star-Topology Enforcement**: Indices are the only primary anchors. All notes must link back to their parent index to prevent visual clutter.
- **Traversal Logic**: If the graph looks like a "pile of shit," traverse the links. If nodes are confusingly connected, break the link and re-anchor to the appropriate Index.
- **Redundancy Audit**: If information is redundant, merge it. If a note is empty, fill it or delete it.
