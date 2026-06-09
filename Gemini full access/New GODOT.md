**Project Vision: The "Storylet-Engine"**

This project focuses on building a Quality-Based Narrative (QBN) engine using Godot 4.x [1]. Unlike traditional linear games, this engine is decoupled from linear flow and driven entirely by state-checking and resource-based data [1]. The core philosophy is strict adherence to a data-driven model: absolutely no game logic should be hardcoded [1]. 

The atomic unit of the story will be the **Storylet**, defined as a Godot Resource containing identifiers, unlock requirements, narrative content, categorization tags, and world-state effects [2]. 

**Core Architecture Principles**
*   **Centralized State:** A `WorldState` Singleton (e.g., `GameState.gd`) will act as the single source of truth for all player attributes, inventory, and progression flags, persisting across the entire game [3].
*   **Decoupled UI (No "Spaghetti" Code):** Narrative logic must never be placed inside UI nodes [4]. The UI will be dynamic—dynamically generating choice buttons in a `VBoxContainer`—and will only listen for signals and display data provided by the `StoryManager` [4, 5].
*   **Data Separation:** All narrative text must be stored in JSON or Resource files [4]. This ensures that the story can be updated in the future without needing to recompile the game executable [4].

**Immediate Action Items**
To begin executing this technical roadmap, please proceed with the following initial steps [4]:
1. Review the `StoryManager` architecture, which acts as the parsing brain of the engine [3, 4].
2. Set up the initial Godot project [4].
3. Create the `Storylet.gd` resource script [4].
4. Provide confirmation once your prototype is capable of parsing a JSON file into a list of usable resources [4].