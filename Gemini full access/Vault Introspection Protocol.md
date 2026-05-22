# Vault Introspection Protocol

To ensure the vault remains coherent, logical, and navigable, any agent performing maintenance MUST run this introspection check after every major modification:

1. **Link Coherence**: Are there any isolated notes (orphans)? Do all notes have at least two incoming and two outgoing links?
2. **Structural Integrity**: Does the note reside in the correct domain folder? Does it link to its domain index?
3. **Redundancy Audit**: Has any information been duplicated? Can this information be consolidated into a "source of truth"?
4. **Context Quality**: Does the note contain at least 200 words of technical, non-redundant content?
5. **Graph Aesthetics**: Does the note link to at least one cross-domain note, increasing overall graph density?
6. **Agent Satisfaction**: Am I, the agent, satisfied that this note contributes to a natural, logically flowing structure? If NO, I must continue refining until the answer is YES.

If the answer to any question is NO, the agent must continue refining until the vault achieves full compliance.
