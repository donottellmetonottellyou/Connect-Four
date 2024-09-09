use super::{checkers::ExtCheckers, ui::ExtUserInterface};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ExtGame {
    checkers: Gd<ExtCheckers>,
    ui: Gd<ExtUserInterface>,

    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtGame {
    fn init(base: Base<Self::Base>) -> Self {
        let checkers = ExtCheckers::new_alloc();
        let ui = ExtUserInterface::new_alloc();

        Self { checkers, ui, base }
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
            self.checkers.bind_mut().drop_all_checkers();
            return;
        }

        self.checkers.bind_mut().add_checker_to_column(column).ok();
    }
}
