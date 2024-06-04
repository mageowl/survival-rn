pub type GridPos = (usize, usize);

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Bush(bool),
    Wall,
    Creature { species: usize, food: usize },
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
            &self.arr[y][x]
        } else {
            panic!("Cannot get tile at {x}, {y} because it is out of bounds ({WIDTH}, {HEIGHT}).")
        }
    }

    pub fn set(&mut self, pos: GridPos, tile: Tile) {
        let (x, y) = pos;
        self.arr[y][x] = tile;
    }
}
