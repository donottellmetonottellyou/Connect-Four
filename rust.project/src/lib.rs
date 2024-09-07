mod game_control;
mod grid;
mod ui;

use godot::prelude::*;

struct ConnectFourExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ConnectFourExtensionLibrary {}
