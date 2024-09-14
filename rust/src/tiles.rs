use godot::{
    engine::{ITileMap, TileMap},
    obj::WithBaseField,
    prelude::*,
};

use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=TileMap)]
pub struct ExtTileMap {
    is_highlighted: bool,

    base: Base<TileMap>,
}
#[godot_api]
impl ITileMap for ExtTileMap {
    fn init(base: Base<Self::Base>) -> Self {
        let is_highlighted = false;

        Self {
            is_highlighted,
            base,
        }
    }
}
impl ExtTileMap {
    const SOURCE_ATLAS: i32 = 0;
    const HIGHLIGHTED_COORDINATES: Vector2i = Vector2i { x: 1, y: 0 };
    const UNHIGHLIGHTED_COORDINATES: Vector2i = Vector2i { x: 0, y: 0 };

    pub fn highlight_tiles(&mut self, coordinates: HashSet<(usize, usize)>) -> Result<(), ()> {
        if self.is_highlighted {
            return Err(());
        }

        coordinates
            .into_iter()
            .map(|coordinates| (coordinates.0 + 2, coordinates.1 + 2))
            .for_each(|coordinates| {
                self.base_mut()
                    .set_cell_ex(
                        1,
                        Vector2i {
                            x: coordinates.1 as i32,
                            y: coordinates.0 as i32,
                        },
                    )
                    .source_id(Self::SOURCE_ATLAS)
                    .atlas_coords(Self::HIGHLIGHTED_COORDINATES)
                    .done()
            });

        self.is_highlighted = true;
        Ok(())
    }

    pub fn reset_tiles(&mut self) {
        for row in (0..6).map(|row| row + 2) {
            for column in (0..7).map(|column| column + 2) {
                self.base_mut()
                    .set_cell_ex(1, Vector2i { x: column, y: row })
                    .source_id(Self::SOURCE_ATLAS)
                    .atlas_coords(Self::UNHIGHLIGHTED_COORDINATES)
                    .done();
            }
        }

        self.is_highlighted = false;
    }
}
