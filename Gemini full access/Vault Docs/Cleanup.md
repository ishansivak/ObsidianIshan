# Vault Cleanup Protocol

This protocol is the definitive guide for maintaining vault health, eliminating redundancies, and ensuring a clean, navigable graph view.

## Core Mandates
1. **Zero Tolerance for Clutter**: Empty notes must be deleted or merged immediately.
2. **Context Enrichment**: All research notes must maintain a minimum of 200 words of substantive technical content.
3. **Graph Readability**: Maintain a star-topology centered on domain indices (`[[Biological Index]]`, `[[Protocol Index]]`, `[[Infrastructure Index]]`).
4. **Bidirectional Integrity**: Every note must have at least two incoming and two outgoing links. No orphans. No dead ends.
5. **Redundancy Purge**: Consolidate duplicate information into a single source of truth.

## Execution Steps
1. **Audit**: Read the note.
2. **Assess**: Is it empty? Does it duplicate info?
3. **Act**: Delete/Merge/Expand. Save discarded content to `[[tempTrash]]`.
4. **Link**: Ensure valid bidirectional connections.
5. **Log**: Record reasoning in `[[Edit Explanations/Maintenance Run Report 2026-05-23.md]]`.
