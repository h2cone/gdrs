use godot::init::{ExtensionLibrary, gdextension};

mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
