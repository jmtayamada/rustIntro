use godot::prelude::*;

struct GodotRust;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRust {}
