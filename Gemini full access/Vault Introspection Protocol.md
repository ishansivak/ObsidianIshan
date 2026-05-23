# Vault Introspection Protocol

Before completing any maintenance run, an agent must answer the following questions to ensure vault coherence:

1. **Link Integrity**: Are there any orphaned notes or dead ends? Do all notes have at least two incoming/outgoing links?
2. **Redundancy Audit**: Have I consolidated all duplicate information into a single "source of truth"? Have I moved all discarded content to `[[tempTrash]]`?
3. **Context Depth**: Do all notes meet the 200-word substantive context threshold?
4. **Structural Coherence**: Does the current folder structure and index hierarchy reflect the natural flow of information?
5. **Historical Respect**: Have I reviewed past relevant conversations in `gemini-scribe/` to ensure my changes align with past decisions?

If the answer to any of these is "No," the agent must continue the maintenance process until all criteria are met.
