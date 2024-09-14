mod checkers;
mod game_control;
mod tiles;
mod ui;

use godot::prelude::*;

struct ConnectFourExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ConnectFourExtensionLibrary {}
