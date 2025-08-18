use godot::prelude::*;

struct RustExtension;

pub mod gdrust_trinkets;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
