use super::{checkers::ConnectFourGrid, ui::ConnectFourUserInterface};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ConnectFourGame {
    grid: Gd<ConnectFourGrid>,
    ui: Gd<ConnectFourUserInterface>,

    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ConnectFourGame {
    fn init(base: Base<Self::Base>) -> Self {
        let grid = ConnectFourGrid::new_alloc();
        let ui = ConnectFourUserInterface::new_alloc();

        Self { grid, ui, base }
    }

    fn ready(&mut self) {
        let nodes = [self.grid.clone().upcast(), self.ui.clone().upcast()];

        for node in nodes {
            self.base_mut().add_child(node);
        }
    }
}
#[godot_api]
impl ConnectFourGame {
    pub fn play_column(&mut self, column: usize) {
        self.grid.bind_mut().add_checker_to_column(column).ok();
    }
}
