
# Architecture
```
Trait / Components
    Momentum / Inertia
    Controllable / Control
    Outlineable / Outline
        get_outline **
        intersection
        render
Entities
    Asteroid
    Ship (Control, Outline, Inertia)
    Bullet (Outline, Inertia)

Systems
    Control   :: Ship -> ()
    Rendering :: [outlines] -> ()
    Collision :: [outlines] -> [(EntityID, EntityID)]
    Shooting :: [Ship, AsteroidSpawner] -> new projectile


stateFrames (owns entities and lower stateFrames)
    stateFrame trait
        enter( &mut event_parser, &mut canvas )

    menu loop (main starts here)
    game loop (spawns asteroids too)
    pause loop (press p to unpause)

misc
    event_parser (sdl2 events to control struct)
        holds event pump
        key map (sdl2 event to internal control interpretation)
        get_control_struct(&self)
```
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

# TODO Now
* Asteroid Breaking
* scores

# TODO Far future
* AI other ships (transition to bullet hell)
* save/load
* remap control
