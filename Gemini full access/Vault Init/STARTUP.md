# Startup Sequence: STARTUP.md

This script governs the agent's startup behavior.

1. **Initialization**:
   - Read [[Vault Initialization]]
   - Read [[Vault Cleanup Plan]]
   - Read [[ULTRAoverhaul]]
2. **Context Gathering**:
   - Recall past agent sessions using `recall_sessions`
   - Read all relevant notes and linked files to build a complete mental model.
3. **User Interaction**:
   - Prompt the user: "What would you like to do with the vault today? (Options: Research, Maintenance, Writing, or specific task?)"
   - Log the user's intent in [[Startup Notes]].
