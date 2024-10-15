use godot::prelude::*;

struct GodotRust;

mod player2d;
mod attack;
mod state_machine;
mod player;
mod state;
mod character_controller;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRust {}
