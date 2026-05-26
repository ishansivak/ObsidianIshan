System Architecture and Implementation Prompt: Rust-based LLM Frontend
Project Overview
You are tasked with building an open-source, browser-based frontend for interacting with Large Language Models (LLMs) like Gemini (remote API) and Gemma (local via Ollama). The entire application must be written in Rust using the Dioxus framework, targeting a WebAssembly (Wasm) single-page application (SPA).
Core Requirements
Framework: Use Rust with Dioxus (dioxus-web).
State Management: Utilize Dioxus hooks (e.g., use_signal, use_coroutine) to manage chat history, streaming text states, and model selection.
LLM Integration:
Create an extensible API client using reqwest to handle REST calls.
Implement an interface for the Google Gemini API (handling API keys via user input).
Implement an interface for local models (e.g., Gemma) running via Ollama, optimized for local Apple Silicon (macOS M-series) environments.
User Interface:
A clean, modern chat interface with a unified message thread.
A settings panel to toggle between models (Local Ollama vs. Remote Gemini), configure endpoints, and input system prompts.
Markdown rendering for LLM responses using a Rust crate like pulldown-cmark.
Step-by-Step Implementation Plan
Project Setup: Initialize a new Dioxus web project. Configure the Cargo.toml with necessary dependencies: dioxus, reqwest, serde, serde_json, pulldown-cmark, and Wasm execution crates.
UI Components: Create modular Dioxus components:
ChatWindow: Iterates over state and displays the list of messages.
MessageBubble: Renders individual user/assistant messages with Markdown support.
InputArea: Text area with a submit button, handling "Enter" to send, alongside a loading indicator.
SettingsSidebar: Controls to select the active model and input the API key or local Ollama URL (defaulting to http://localhost:11434).
API Logic: Write asynchronous functions to handle POST requests to the selected LLM backend. Ensure proper Wasm-compatible error handling and JSON parsing.
State Wiring: Connect the UI to the API logic using Dioxus coroutines so the Wasm main thread does not block while waiting for the LLM's response.
Instructions for Code Generation
Generate the complete Cargo.toml and the src/main.rs (along with any necessary module files). Ensure the code is idiomatic Rust, follows Dioxus 0.5+ paradigms, is heavily commented, and is ready to be compiled with the Dioxus CLI (dx serve). Focus on type safety and handling Wasm-specific async execution properly.