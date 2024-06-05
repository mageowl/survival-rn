use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Index, IndexMut, Sub},
};

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

impl Into<IPos> for Pos {
    fn into(self) -> IPos {
        IPos(self.0 as isize, self.1 as isize)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy)]
pub struct IPos(pub isize, pub isize);

impl Add<IPos> for IPos {
    type Output = IPos;

    fn add(self, rhs: IPos) -> Self::Output {
        IPos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<IPos> for IPos {
    type Output = IPos;

    fn sub(self, rhs: IPos) -> Self::Output {
        IPos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl TryInto<Pos> for IPos {
    type Error = ();

    fn try_into(self) -> Result<Pos, ()> {
        if self.0 >= 0 && self.1 >= 0 {
            Ok(Pos(self.0 as usize, self.1 as usize))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::OutOfBounds => write!(f, "X"),
            Self::Bush(true) => write!(f, "%"),
            Self::Bush(false) => write!(f, "/"),
            Self::Wall(_) => write!(f, "#"),
            Self::Creature { .. } => write!(f, "@"),
        }
    }
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

    pub fn slice<const SLICE_WIDTH: usize, const SLICE_HEIGHT: usize>(
        &self,
        Pos(x, y): Pos,
    ) -> [[Tile; SLICE_WIDTH]; SLICE_HEIGHT] {
        let mut slice = [[Tile::OutOfBounds; SLICE_WIDTH]; SLICE_HEIGHT];

        for (i, row) in self.arr[y..y + SLICE_HEIGHT].iter().enumerate() {
            slice[i].copy_from_slice(&row[x..x + SLICE_WIDTH]);
        }

        slice
    }

    pub fn i_slice<const SLICE_WIDTH: usize, const SLICE_HEIGHT: usize>(
        &self,
        IPos(x, y): IPos,
    ) -> [[Tile; SLICE_WIDTH]; SLICE_HEIGHT] {
        let mut slice = [[Tile::OutOfBounds; SLICE_WIDTH]; SLICE_HEIGHT];
        let (ox, oy) = (-x.max(0) as usize, -y.max(0) as usize);
        let (cx, cy) = (x.max(0) as usize, y.max(0) as usize);
        let (mx, my) = (
            (x + SLICE_WIDTH as isize).max(0) as usize,
            (y + SLICE_HEIGHT as isize).max(0) as usize,
        );

        for (i, row) in self.arr[cy..my].iter().enumerate() {
            for (j, tile) in row[cx..mx].iter().enumerate() {
                slice[oy + i][ox + j] = *tile;
            }
        }

        slice
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
