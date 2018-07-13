# Done so far
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

# TODO Now
* Level system
* AI System -> other ships -> transition to bullet hell

# TODO Far future
* Refactor
    * EntityManager (holds entity_map and component owner sets)
    * Update systems to take specific parts of IO, rather than whole controller
    * Spawn projectiles in `impl Projectile` rather than in entity
* Mouse based aiming and absolute controls
* Expandable ship (attaching other weapons)
* save/load
* remap control
