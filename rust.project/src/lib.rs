mod checkers;
mod game_control;
mod ui;

use godot::prelude::*;

struct ConnectFourExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ConnectFourExtensionLibrary {}
