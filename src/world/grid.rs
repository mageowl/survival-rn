use std::ops::{Add, Index, IndexMut};

use raylib::{color::Color, drawing::RaylibDraw};

use crate::{assets::Assets, util::TILE_SIZE};

#[derive(Clone, Copy)]
pub struct Pos(pub usize, pub usize);

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    OutOfBounds,
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

    pub fn render(&self, d: &mut impl RaylibDraw, assets: &Assets) {
        for (y, row) in self.arr.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Empty | Tile::OutOfBounds => (),
                    Tile::Bush(has_food) => d.draw_texture(
                        if *has_food {
                            &assets.bush_berries
                        } else {
                            &assets.bush
                        },
                        x as i32 * TILE_SIZE,
                        y as i32 * TILE_SIZE,
                        Color::WHITE,
                    ),
                    Tile::Wall(_species) => todo!(),
                    Tile::Creature { color, food, .. } => {
                        d.draw_texture(
                            &assets.agent,
                            x as i32 * TILE_SIZE,
                            y as i32 * TILE_SIZE,
                            color,
                        );
                        d.draw_text(
                            &food.to_string(),
                            x as i32 * TILE_SIZE,
                            y as i32 * TILE_SIZE - 4,
                            6,
                            Color::BLACK,
                        )
                    }
                }
            }
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<Pos> for Grid<WIDTH, HEIGHT> {
    fn index_mut(&mut self, Pos(x, y): Pos) -> &mut Self::Output {
        if x >= WIDTH || y >= HEIGHT {
            panic!("Cannot set tile at {x}, {y} because it is out of bounds ({WIDTH}, {HEIGHT}).")
        } else {
            &mut self.arr[y][x]
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<Pos> for Grid<WIDTH, HEIGHT> {
    type Output = Tile;

    fn index(&self, Pos(x, y): Pos) -> &Self::Output {
        if x >= WIDTH || y >= HEIGHT {
            &Tile::OutOfBounds
        } else {
            &self.arr[y][x]
        }
    }
}
