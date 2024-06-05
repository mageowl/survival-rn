use std::{cell::RefCell, rc::Rc};

use raylib::color::Color;

use super::grid::{Grid, IPos, Pos, Tile};
use crate::{
    util::{GRID_HEIGHT, GRID_WIDTH},
    world::World,
};

pub struct SpeciesConfig {
    /// Color of creatures
    pub color: Color,
    /// Number of creatures in a pack
    pub num_creatures: usize,
    /// Number of packs
    pub num_packs: usize,
}

pub struct Species {
    pub id: usize,
    pub members: Vec<Pos>,
    pub color: Color,
    grid: Rc<RefCell<Grid<GRID_WIDTH, GRID_HEIGHT>>>,
}

impl Species {
    pub fn new(id: usize, world: &World, color: Color) -> Self {
        Self {
            id,
            members: Vec::new(),
            grid: world.grid.clone(),
            color,
        }
    }

    pub fn get_view_slice(&self, index: usize) -> [[Tile; 7]; 7] {
        let i_pos: IPos = Into::<IPos>::into(self.members[0]) - IPos(3, 3);

        if let Ok(pos) = i_pos.try_into() {
            self.grid.borrow().slice(pos)
        } else {
            self.grid.borrow().i_slice(i_pos)
        }
    }

    pub fn get_food(&self, index: usize) -> usize {
        if let Tile::Creature { food, .. } = self.grid.borrow()[self.members[index]] {
            food
        } else {
            panic!(
                "Expected creature at position {}. (Trying to access amount of food)",
                self.members[index]
            );
        }
    }
}
