use godot::prelude::*;

mod rust_hello_world;

struct HelloRustGodot;

#[gdextension]
unsafe impl ExtensionLibrary for HelloRustGodot {}
