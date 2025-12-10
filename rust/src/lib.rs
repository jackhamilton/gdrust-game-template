use godot::prelude::*;

struct RustExtension;

pub mod gdrust_trinkets;
pub mod sample_ui;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
