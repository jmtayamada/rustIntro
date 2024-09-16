use godot::prelude::*;

struct GodotRust;

mod player2d;
mod player;
mod manager;
mod attack;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRust {}
