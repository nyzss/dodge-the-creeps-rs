mod mob;
mod player;

use godot::prelude::*;

struct DodgeTheCreepsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreepsExtension {}
