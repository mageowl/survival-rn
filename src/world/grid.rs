use raylib::{color::Color, drawing::RaylibDraw};

use crate::assets::Assets;

pub type GridPos = (usize, usize);

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Bush(bool),
    Wall(usize),
    Creature {
        species: usize,
        color: Color,
        food: usize,
    },
}

pub struct Grid<const WIDTH: usize, const HEIGHT: usize> {
    arr: [[Tile; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid<WIDTH, HEIGHT> {
    pub fn empty() -> Self {
        Grid {
            arr: [[Tile::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn at(&self, pos: GridPos) -> &Tile {
        let (x, y) = pos;

        if x >= WIDTH || y >= HEIGHT {
            panic!("Cannot get tile at {x}, {y} because it is out of bounds ({WIDTH}, {HEIGHT}).")
        } else {
            &self.arr[y][x]
        }
    }

    pub fn set(&mut self, pos: GridPos, tile: Tile) {
        let (x, y) = pos;
        self.arr[y][x] = tile;
    }

    pub fn render(&self, d: &mut impl RaylibDraw, assets: &Assets) {
        for (y, row) in self.arr.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Empty => (),
                    Tile::Bush(has_food) => d.draw_texture(
                        if *has_food {
                            &assets.bush_berries
                        } else {
                            &assets.bush
                        },
                        x as i32 * 16,
                        y as i32 * 16,
                        Color::WHITE,
                    ),
                    Tile::Wall(_species) => todo!(),
                    Tile::Creature { color, food, .. } => {
                        d.draw_texture(&assets.agent, x as i32 * 16, y as i32 * 16, color);
                        d.draw_text(
                            &format!("food: {food}"),
                            x as i32,
                            y as i32 - 8,
                            18,
                            Color::BLACK,
                        )
                    }
                }
            }
        }
    }
}
