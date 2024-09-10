use super::{checkers::ExtCheckers, ui::ExtUserInterface};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ExtGame {
    is_finished: bool,

    checkers: Gd<ExtCheckers>,
    ui: Gd<ExtUserInterface>,

    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtGame {
    fn init(base: Base<Self::Base>) -> Self {
        let is_finished = false;

        let checkers = ExtCheckers::new_alloc();
        let ui = ExtUserInterface::new_alloc();

        Self {
            is_finished,
            checkers,
            ui,
            base,
        }
    }

    fn ready(&mut self) {
        let nodes = [self.checkers.clone().upcast(), self.ui.clone().upcast()];

        for node in nodes {
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
            self.is_finished = self
                .checkers
                .bind()
                .find_connected_fours(played.0, played.1)
                .is_some()
        };
    }

    #[inline]
    fn restart_game(&mut self) {
        self.checkers.bind_mut().drop_all_checkers();
        self.is_finished = false;
    }
}
