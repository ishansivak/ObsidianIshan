# Vault Initialization

This folder contains the core files required for an agent to boot up and understand the vault's governance, structure, and current state.

## Core Directives
1. **Exhaustive Review**: Before any structural change, the agent MUST perform an exhaustive read-through of every single note in the vault.
2. **Connectivity Audit**: Every note must have at least two incoming and two outgoing links to relevant indices or related research notes. No orphans. No dead ends.
3. **Redundancy Purge**: Consolidate duplicates into a single "source of truth".
4. **Context Enrichment**: All notes must contain a minimum of 200 words of substantive, technical context. **ABJECT FAILURE TOLERANCE IS ZERO: Empty or sub-200-word notes are prohibited.**
5. **Graph Density**: Cross-domain links between `Biological Networks`, `Protocol Engineering`, and `Hardware Infrastructure` must be explicitly added.
6. **Graph Readability**: The graph view must be prioritized as an aesthetic, elegant, and navigable interface. Avoid excessive clustering; maintain clear, logical clusters around core indices. **If the graph view looks like a pile of shit, it is a failure of the agent.**
7. **Data Respect**: Never overwrite or delete information without first verifying it is truly redundant. 
8. **Cleanup Protocol**: All deleted/discarded materials MUST be moved to `[[tempTrash]]` instead of being permanently deleted.
9. **Historical Context**: Agents must read all past agent conversations in the `gemini-scribe/` subfolder before initiating any structural changes.
10. **Traversal Logic**: If you are confused by the structure, traverse the links to understand the notes. If the graph still appears cluttered or illogical, change the structure immediately to improve clarity. "Be like water."
11. **Measure Twice, Cut Once**: Always read the entire vault content before cutting. If you cut, be ruthless but precise.
12. **Visualization & Tooling**: Actively create and maintain Kanban boards and other visualization tools where useful. If a visualization tool helps clarify the structure, build it.
13. **Logging**: Every maintenance run MUST log its changes and the reasoning behind those changes in the `Edit Explanations/` folder.

---

## Vault Management Directives (Transferred from README)
- **Core Directives**: 
    1. "Do more. More context, more structure. Less redundant crap. More logic. Better links. Less dead links. More navigable in the graph view. Think for a long time before taking actions, but take lots of action."
    2. "You didn't make the necessary adjustments yet. do more. more context more structure. less redundant crap. more logic. better links. less dead links. more navigable in the graph view. go go go. think for a long time before taking actions. but take lots of action. also make sure that the readme keeps in store a bunch of context about the vault, including this exact prompt for future use. Append more strict alternatives to this prompt in the readme so users can make sure you actually go back and read every. single. note. now get started. dont be lazy. take your time. think. then act ruthlessly and cunningly. be like water. let the concepts and structures flow naturally as opposed to artificial knowledge structures."
    3. "When modifying this vault, you must act with extreme respect for existing data. Before adding, deleting, or restructuring, you must read all affected notes to understand their nuance. Do not delete or add stuff before thinking long and hard. Act ruthlessly against redundancy, but with surgical precision to preserve the underlying intellectual capital."
    4. **Traversal Logic**: If you are confused by the structure, traverse the links to understand the notes. If the graph still appears cluttered or illogical, change the structure immediately to improve clarity. "Be like water."
