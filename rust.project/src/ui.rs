use super::game_control::ExtGame;

use godot::{
    engine::{HBoxContainer, IHBoxContainer, ITextureButton, TextureButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=HBoxContainer, init)]
pub struct ExtUserInterface {
    base: Base<HBoxContainer>,
}
#[godot_api]
impl IHBoxContainer for ExtUserInterface {
    fn ready(&mut self) {
        self.base_mut()
            .add_theme_constant_override("separation".into(), 0);

        for i in 0..7 {
            let mut column = ExtColumnButton::new_alloc();
            column.bind_mut().column = i;
            self.base_mut().add_child(column.upcast());
        }
    }
}
#[godot_api]
impl ExtUserInterface {
    fn select_column(&self, column: usize) {
        let mut game_control: Gd<ExtGame> = self.base().get_parent().unwrap().cast();
        game_control.bind_mut().play_column(column);
    }
}

#[derive(GodotClass)]
#[class(base=TextureButton, init)]
struct ExtColumnButton {
    column: usize,

    base: Base<TextureButton>,
}
#[godot_api]
impl ITextureButton for ExtColumnButton {
    fn ready(&mut self) {
        self.base_mut()
            .set_custom_minimum_size(Vector2 { x: 16.0, y: 96.0 });

        let on_pressed = self.to_gd().callable("on_pressed");
        self.to_gd().connect("pressed".into(), on_pressed);
    }
}
#[godot_api]
impl ExtColumnButton {
    #[func]
    fn on_pressed(&self) {
        let parent: Gd<ExtUserInterface> = self.base().get_parent().unwrap().cast();
        parent.bind().select_column(self.column);
    }
}
