# lau-agent-shell

> PLATO agent shell — an agent in a room, with a game character on the outside

## What This Does

PLATO agent shell — an agent in a room, with a game character on the outside. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-agent-shell
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_agent_shell::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub struct AgentState 
pub enum AgentPhase 
pub struct CharacterAppearance 
pub enum CharacterAnimation 
pub struct CharacterAction 
pub enum ActionKind 
pub struct AgentShell 
    pub fn new(id: &str, room_id: &str) -> Self 
    pub fn observe(&mut self, value: f64, confidence: f64) 
    pub fn predict(&mut self, predicted: f64, actual: f64) 
    pub fn dissolve(&mut self) 
    pub fn finish_dissolve(&mut self) 
    pub fn speak(&mut self, text: &str) 
    pub fn teach(&mut self, target_id: &str, knowledge: f64) 
    pub fn flush_actions(&mut self) -> Vec<CharacterAction> 
    pub fn character_card(&self) -> String 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**22 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
