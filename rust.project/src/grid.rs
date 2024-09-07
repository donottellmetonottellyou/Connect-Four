use godot::{
    engine::{CanvasGroup, ICanvasGroup, ISprite2D, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CanvasGroup)]
pub struct ConnectFourGrid {
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
