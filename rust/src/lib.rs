use godot::init::{ExtensionLibrary, gdextension};

mod main_scene;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
