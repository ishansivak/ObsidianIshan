# Visualization Engine Protocol

This protocol governs the generation of visual structures to improve vault navigation for humans, agents, and external tools like NotebookLM.

## Visualization Mandate
Agents must regularly analyze note clusters and index density to propose and implement visual aids.

## Current Visualization Tools
- **Kanban Boards**: Used for managing active research tasks, pending audits, and structural overhauls. 
- **Thematic Maps**: Nodes are clustered by domain (Biological, Protocol, Infrastructure) to ensure the graph view is intuitive and balanced.

## Visualization Suggestions
- **Domain Flow Diagrams**: Create Mermaid diagrams within index notes (e.g., [[Biological Index]]) to visualize how biological principles flow into protocol engineering.
- **Progress Trackers**: Use checklists within [[Vault Cleanup Plan]] and [[Final Overhaul Plan]] to visualize maintenance progress.

## Agent Instructions
1. Analyze notes for high-traffic concepts.
2. If a concept connects 3+ domains, suggest a visual diagram or table.
3. If the graph view looks cluttered, create a new Kanban board to offload complexity.
