# Vault Cleanup Plan

This document outlines the systematic approach for maintaining and optimizing the vault's structure and content.

## Current Maintenance Strategy
1. **Iterative Audit**: Every maintenance session must include a review of a subset of notes to ensure compliance with the 200-word context threshold.
2. **Link Density Verification**: Every note must maintain at least two incoming and two outgoing links to relevant indices or related research notes.
3. **Graph Optimization**: Actively look for opportunities to link across domain categories (Biological, Protocol, Infrastructure) to improve graph navigation.
4. **README Synchronization**: When structural changes occur (new folders, index changes), update the `[[README]]` to reflect the new state.

## Pending Tasks
- [ ] Conduct a full audit of all notes for 200-word context compliance.
- [ ] Verify bidirectional link integrity across all indices.
- [ ] Refine graph aesthetics by standardizing nomenclature in note titles and index structure.

## Graph Aesthetic & Readability Protocol (Added 2026-05-23)
- **Star-Topology Enforcement**: Indices are the only primary anchors. All notes must link back to their parent index to prevent visual clutter.
- **Traversal Logic**: If the graph looks like a "pile of shit," traverse the links. If nodes are confusingly connected, break the link and re-anchor to the appropriate Index.
- **Redundancy Audit**: If information is redundant, merge it. If a note is empty, fill it or delete it.
