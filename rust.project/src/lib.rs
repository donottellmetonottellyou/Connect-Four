use godot::{
    engine::{
        Button, CanvasGroup, HBoxContainer, IButton, ICanvasGroup, IHBoxContainer, ISprite2D,
        ITextureButton, InputEvent, Sprite2D, TextureButton,
    },
    prelude::*,
};
use rand::prelude::*;

struct ConnectFourExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ConnectFourExtensionLibrary {}

#[derive(GodotClass)]
#[class(base=HBoxContainer, init)]
struct ConnectFourUserInterface {
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

#[derive(GodotClass)]
#[class(base=CanvasGroup)]
struct ConnectFourGrid {
    red_checkers: Vec<Gd<ConnectFourChecker>>,
    yellow_checkers: Vec<Gd<ConnectFourChecker>>,
    grid: [[Option<Gd<ConnectFourChecker>>; 7]; 6],

    base: Base<CanvasGroup>,
}
#[godot_api]
impl ICanvasGroup for ConnectFourGrid {
    fn init(base: Base<Self::Base>) -> Self {
        let mut red_checkers = Vec::with_capacity(Self::PLAYER_CHECKER_NUMBER);
        let mut yellow_checkers = Vec::with_capacity(Self::PLAYER_CHECKER_NUMBER);
        for _ in 0..Self::PLAYER_CHECKER_NUMBER {
            red_checkers.push(ConnectFourChecker::new_alloc());
            yellow_checkers.push({
                let mut yellow_checker = ConnectFourChecker::new_alloc();
                yellow_checker.bind_mut().is_yellow = true;
                yellow_checker
            });
        }

        Self {
            red_checkers,
            yellow_checkers,
            grid: std::array::from_fn(|_| std::array::from_fn(|_| None)),

            base,
        }
    }

    fn process(&mut self, _delta: f64) {
        let column = thread_rng().gen_range(0..7);
        let is_yellow = thread_rng().gen_bool(0.5);

        self.add_checker_to_column(column as usize, is_yellow).ok();
    }
}
impl ConnectFourGrid {
    const PLAYER_CHECKER_NUMBER: usize = 21;
    const GRID_CELL_SIZE: usize = 16;

    fn add_checker_to_column(&mut self, column: usize, is_yellow: bool) -> Result<(), ()> {
        let mut checker = match (
            is_yellow,
            self.yellow_checkers.len() - self.red_checkers.len(),
        ) {
            (false, 0) => self.red_checkers.pop(),
            (true, 1) => self.yellow_checkers.pop(),
            _ => None,
        }
        .ok_or(())?;

        let row = match self.put_in_column(column, checker.clone()) {
            Ok(row) => row,
            Err(_) => {
                match is_yellow {
                    false => self.red_checkers.push(checker),
                    true => self.yellow_checkers.push(checker),
                };
                return Err(());
            }
        };

        checker.set_position(Vector2 {
            x: (column * Self::GRID_CELL_SIZE) as f32,
            y: -(Self::GRID_CELL_SIZE as isize) as f32,
        });

        checker.bind_mut().target = Some(Vector2 {
            x: (column * Self::GRID_CELL_SIZE) as f32,
            y: (row * Self::GRID_CELL_SIZE) as f32,
        });

        self.base_mut().add_child(checker.upcast());

        Ok(())
    }

    #[inline]
    fn put_in_column(
        &mut self,
        column: usize,
        checker: Gd<ConnectFourChecker>,
    ) -> Result<usize, ()> {
        for (i, row) in self.grid.iter_mut().enumerate().rev() {
            match row.get_mut(column).ok_or(())? {
                Some(_) => (),
                cell => {
                    *cell = Some(checker);
                    return Ok(i);
                }
            }
        }

        Err(())
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D, init)]
struct ConnectFourChecker {
    is_yellow: bool,
    target: Option<Vector2>,

    base: Base<Sprite2D>,
}
#[godot_api]
impl ISprite2D for ConnectFourChecker {
    fn ready(&mut self) {
        let is_yellow = self.is_yellow;
        let mut base = self.base_mut();

        base.set_texture(load("res://Assets/checkers.png"));
        base.set_hframes(2);
        base.set_frame(match is_yellow {
            false => 0,
            true => 1,
        })
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(target) = self.target {
            let current_position = self.base().get_position();
            let movement =
                current_position.direction_to(target).normalized_or_zero() * delta as f32 * 256.0;
            self.base_mut().set_position(current_position + movement);
            if (target - self.base().get_position()).length() < 4.0 {
                self.base_mut().set_position(target);
                self.target = None;
            }
        }
    }
}
impl ConnectFourChecker {
    #[inline]
    fn is_yellow(&self) -> bool {
        self.is_yellow
    }
}
