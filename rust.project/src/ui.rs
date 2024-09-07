use godot::{
    engine::{HBoxContainer, IHBoxContainer, ITextureButton, TextureButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=HBoxContainer, init)]
pub struct ConnectFourUserInterface {
    base: Base<HBoxContainer>,
}
#[godot_api]
impl IHBoxContainer for ConnectFourUserInterface {
    fn ready(&mut self) {
        self.base_mut().set_position(Vector2 { x: 16.0, y: 16.0 });
        self.base_mut()
            .add_theme_constant_override("separation".into(), 0);

        for i in 0..7 {
            let mut column = ConnectFourColumnButton::new_alloc();
            column.bind_mut().column = i;
            self.base_mut().add_child(column.upcast());
        }
    }
}
#[godot_api]
impl ConnectFourUserInterface {
    fn select_column(&mut self, column: usize) {
        godot_print!("Column {} selected", column);
    }
}

#[derive(GodotClass)]
#[class(base=TextureButton, init)]
struct ConnectFourColumnButton {
    column: usize,

    base: Base<TextureButton>,
}
#[godot_api]
impl ITextureButton for ConnectFourColumnButton {
    fn ready(&mut self) {
        self.base_mut()
            .set_custom_minimum_size(Vector2 { x: 16.0, y: 96.0 });

        let on_pressed = self.to_gd().callable("on_pressed");
        self.to_gd().connect("pressed".into(), on_pressed);
    }
}
#[godot_api]
impl ConnectFourColumnButton {
    #[func]
    fn on_pressed(&mut self) {
        let mut parent: Gd<ConnectFourUserInterface> = self.base().get_parent().unwrap().cast();
        parent.bind_mut().select_column(self.column);
    }
}
