use crate::{checkers::ExtCheckers, tiles::ExtTileMap, ui::ExtUserInterface};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ExtGame {
    is_finished: bool,

    checkers: Gd<ExtCheckers>,
    tiles: Option<Gd<ExtTileMap>>,
    ui: Gd<ExtUserInterface>,

    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtGame {
    fn init(base: Base<Self::Base>) -> Self {
        let is_finished = false;

        let checkers = ExtCheckers::new_alloc();
        let tiles = None;
        let ui = ExtUserInterface::new_alloc();

        Self {
            is_finished,
            checkers,
            tiles,
            ui,
            base,
        }
    }

    fn ready(&mut self) {
        self.tiles = self
            .base()
            .get_child(0)
            .and_then(|child| child.try_cast().ok());

        for node in [self.checkers.clone().upcast(), self.ui.clone().upcast()] {
            self.base_mut().add_child(node);
        }
    }
}
#[godot_api]
impl ExtGame {
    pub fn play_column(&mut self, column: usize) {
        if self.checkers.bind().is_full() {
            self.restart_game();
            return;
        }

        if self.is_finished {
            self.restart_game();
            return;
        }

        let played = self.checkers.bind_mut().add_checker_to_column(column);
        if let Ok(played) = played {
            match (
                self.checkers
                    .bind()
                    .find_connected_fours(played.0, played.1),
                self.tiles.as_mut(),
            ) {
                (Some(connected_fours), Some(tiles)) => {
                    if let Err(()) = tiles.bind_mut().highlight_tiles(connected_fours) {
                        godot_error!("highlight_tiles() was called twice!");
                    }

                    self.is_finished = true;
                }
                (Some(_), None) => {
                    godot_error!("tilemap was not found, skipping highlight!");

                    self.is_finished = true;
                }
                (None, _) => (),
            };
        };
    }

    #[inline]
    fn restart_game(&mut self) {
        self.checkers.bind_mut().drop_all_checkers();

        if let Some(tiles) = self.tiles.as_mut() {
            tiles.bind_mut().reset_tiles();
        }

        self.is_finished = false;
    }
}
