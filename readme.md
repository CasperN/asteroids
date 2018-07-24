
# What is this?
I'm learning game development by building a game of asteroids without an engine.
I only use `sdl2` for IO purposes, everything else is coded myself (poorly).
My architecture is mostly ECS, most functionality are in the methods of
components while my `entity_manager` brings them together into systems.
I plan on transitioning this to a bullet hell style game. For future projects,
I'll probably learn to use an engine of some kind.

# Progress

## Done so far
* Control system
* Momentum system
* Rendering outlines
* 2d vector helper functions
* Ship (Momentum + Controllable + Render)
* Asteroid (Momentum + Render)
* Asteroid spawning / de-spawning
* Projectiles
* Collision detection system
* Impact physics
* Toggle whether things wrap around the border or stick to it
* Health system
* Pause and over screen
* Shrapnel System - breaking into asteroids and outlines
* Sleep until instant of next frame (as opposed to for fixed duration)
* Heads up display (at least for health)
* Entity Manager
* Level system

## TODO Now
* Targeting System
* Opposing ship movement system

## TODO Far future
* Balancing
* Mouse based aiming and absolute controls
* Expandable ship (attaching other weapons)
* save/load
* remap control
