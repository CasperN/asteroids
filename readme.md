
# Architecture
```
traits (Components)
    Momentum

    Controllable

    Outline
        get_outline **
        intersection
    Display
        Outline **
        render
    System
        update_entities **

entities
    Asteroid
    Ship
    Bullet
    MenuScreen
    PauseScreen
    GameBackgroundScreen

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

# TODO Now
* Asteroid spawning / de-spawning
* Bullets
* Collision system
* Toggle whether things wrap around the border or stick to it


# TODO Far future
* save/load
* scores
* remap control
