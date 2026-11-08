# Fantasy 4X Game

A large-scale turn-based fantasy 4X strategy game built with **Rust** and **Bevy Engine**, inspired by games like _The Battle of Polytopia_ and _Civilization V_.

## About

The game combines classic 4X strategy mechanics with a separate, simultaneous-planning combat phase.

Players develop their civilizations, explore the world, gather resources, construct buildings, research technologies, manage politics, and expand their territories.

The main gameplay revolves around two interconnected phases:

### 1. Civilization Phase

The traditional turn-based strategy phase where players manage their civilizations.

- Explore and expand territories.
- Gather resources and manage the economy.
- Construct buildings and develop cities.
- Research technologies and unlock new possibilities.
- Manage politics and civilization development.
- Recruit, upgrade, and position military units.

### 2. Combat Phase

Combat is a separate strategic phase built around simultaneous decision-making.

Instead of simply attacking with units one after another, players (or AI) plan their combat actions without knowing what their opponents are planning.

During this phase, players coordinate their military units and decide:

- Which enemy units to attack.
- Which allied units to protect.
- Which abilities and buffs to use.
- How to distribute support and defensive actions.
- How to use each unit's available skills and upgrades.

Each military unit has its own set of abilities that expands as it gains experience and levels up.

Once all sides have submitted their decisions, the combat simulation begins. The game resolves all planned actions, calculates their interactions, and determines the outcome of the battle.

After combat resolution, the game returns to the Civilization Phase, where players continue developing their empires.

## Core Concept

**Think together. Commit your orders. Discover the outcome.**

Combat is not just about having the strongest army. It is about predicting your opponent, coordinating your forces, choosing the right abilities, and making decisions without knowing the enemy's intentions.

## Game Modes

- **Singleplayer** — play against AI opponents.
- **Online Multiplayer** — compete against other players.
-

## Technology Stack

- **Rust** — core programming language.
- **Bevy 0.19** — game engine.
- **Serde / RON** — serialization and data management.
- **Noise** — procedural world generation.
- **Voronoice** — Voronoi-based world generation.
- **Faster Poisson** — procedural point distribution.
- **Rand / Rand ChaCha** — random generation.
- **Reqwest** — HTTP communication.

## Status

<em>Early development — core systems and gameplay mechanics are being developed.</em>
