
# Architecture
```
game_components (traits)
    Position
        get_xy **
        get_xy_mut **
    Velocity
        Position **
        get_vxy **
        get_vxy_mut **
        accelerate
        move
    Control
        Velocity **
        control_update
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
# TODO Soon
* Circle Asteroids
* Helper functions for 2d vector math
* Bullets
* Collision system
* Toggle whether things wrap around the border or stick to it


# TODO Far future
* save/load
* scores
* remap control
