use godot::{
    engine::{CanvasGroup, ICanvasGroup, ISprite2D, Sprite2D},
    prelude::*,
};

use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=CanvasGroup)]
pub struct ExtCheckers {
    red_checkers: Vec<Gd<ExtChecker>>,
    yellow_checkers: Vec<Gd<ExtChecker>>,
    grid: [[Option<Gd<ExtChecker>>; 7]; 6],

    base: Base<CanvasGroup>,
}
#[godot_api]
impl ICanvasGroup for ExtCheckers {
    fn init(base: Base<Self::Base>) -> Self {
        let mut red_checkers = Vec::with_capacity(Self::PLAYER_CHECKER_NUMBER);
        let mut yellow_checkers = Vec::with_capacity(Self::PLAYER_CHECKER_NUMBER);
        for _ in 0..Self::PLAYER_CHECKER_NUMBER {
            red_checkers.push(ExtChecker::new_alloc());
            yellow_checkers.push({
                let mut yellow_checker = ExtChecker::new_alloc();
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

    fn ready(&mut self) {
        self.base_mut().set_position(Vector2 { x: 8.0, y: 8.0 });
    }
}
#[godot_api]
impl ExtCheckers {
    const PLAYER_CHECKER_NUMBER: usize = 21;
    const GRID_CELL_SIZE: usize = 16;

    #[inline]
    pub fn is_full(&self) -> bool {
        self.yellow_checkers.is_empty()
    }

    pub fn add_checker_to_column(&mut self, column: usize) -> Result<(usize, usize), ()> {
        let is_yellow = match self.yellow_checkers.len() - self.red_checkers.len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }?;

        let mut checker = match is_yellow {
            false => self.red_checkers.pop(),
            true => self.yellow_checkers.pop(),
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

        Ok((row, column))
    }

    #[inline]
    fn put_in_column(&mut self, column: usize, checker: Gd<ExtChecker>) -> Result<usize, ()> {
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

    pub fn find_connected_fours(
        &self,
        row: usize,
        column: usize,
    ) -> Option<HashSet<(usize, usize)>> {
        let mut connected_fours = HashSet::new();

        static OFFSETS: [[isize; 4]; 4] =
            [[-3, -2, -1, 0], [-2, -1, 0, 1], [-1, 0, 1, 2], [0, 1, 2, 3]];

        let connected_fours_check =
            |accumulator: (bool, Option<Gd<ExtChecker>>), checker: Option<Gd<ExtChecker>>| {
                if !accumulator.0 {
                    return (false, None);
                }

                let checker = match checker {
                    Some(checker) => checker,
                    None => return (false, None),
                };

                let previous_checker = match accumulator.1 {
                    Some(previous_checker) => previous_checker,
                    None => return (true, Some(checker)),
                };

                if checker.bind().is_yellow != previous_checker.bind().is_yellow {
                    return (false, None);
                }

                (true, Some(checker))
            };

        let mut four = Vec::with_capacity(4);

        for column_offsets in OFFSETS {
            four.clear();
            four.extend(column_offsets.iter().filter_map(|offset| {
                self.grid[row].get(usize::try_from(column as isize + offset).ok()?)
            }));

            if four.len() != 4 {
                continue;
            }

            if four
                .iter()
                .copied()
                .cloned()
                .fold((true, None), connected_fours_check)
                .0
            {
                connected_fours.extend(
                    column_offsets
                        .iter()
                        .map(|offset| (row, (column as isize + offset) as usize)),
                );
            }
        }

        for row_offsets in OFFSETS {
            four.clear();
            four.extend(row_offsets.iter().filter_map(|offset| {
                self.grid
                    .get(usize::try_from(row as isize + offset).ok()?)
                    .map(|row| &row[column])
            }));

            if four.len() != 4 {
                continue;
            }

            if four
                .iter()
                .copied()
                .cloned()
                .fold((true, None), connected_fours_check)
                .0
            {
                connected_fours.extend(
                    row_offsets
                        .iter()
                        .map(|offset| ((row as isize + offset) as usize, column)),
                )
            }
        }

        for diagonal_flip in [-1, 1] {
            for diagonal_offsets in OFFSETS {
                four.clear();
                four.extend(diagonal_offsets.iter().filter_map(|offset| {
                    self.grid
                        .get(usize::try_from(row as isize + offset * diagonal_flip).ok()?)?
                        .get(usize::try_from(column as isize + offset).ok()?)
                }));

                if four.len() != 4 {
                    continue;
                }

                if four
                    .iter()
                    .copied()
                    .cloned()
                    .fold((true, None), connected_fours_check)
                    .0
                {
                    connected_fours.extend(diagonal_offsets.iter().map(|offset| {
                        (
                            (row as isize + offset * diagonal_flip) as usize,
                            (column as isize + offset) as usize,
                        )
                    }))
                }
            }
        }

        if connected_fours.is_empty() {
            return None;
        }

        Some(connected_fours)
    }

    pub fn drop_all_checkers(&mut self) {
        for checker in self.grid.iter_mut().flat_map(|row| row.iter_mut()) {
            if let Some(checker) = checker.take() {
                let is_yellow = checker.bind().is_yellow;

                match is_yellow {
                    false => self.red_checkers.push(checker),
                    true => self.yellow_checkers.push(checker),
                }
            }
        }

        let children = self.base().get_children();

        for child in children.iter_shared() {
            self.base_mut().remove_child(child);
        }
    }
}
impl Drop for ExtCheckers {
    fn drop(&mut self) {
        for checker in self
            .red_checkers
            .drain(..)
            .chain(self.yellow_checkers.drain(..))
        {
            checker.free();
        }
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D, init)]
struct ExtChecker {
    is_yellow: bool,
    target: Option<Vector2>,

    base: Base<Sprite2D>,
}
#[godot_api]
impl ISprite2D for ExtChecker {
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
