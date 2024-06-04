use std::{cell::RefCell, rc::Rc};

use raylib::color::Color;

use super::grid::{Grid, Pos};
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
    pub(super) fn new(id: usize, world: &World, color: Color) -> Self {
        Self {
            id,
            members: Vec::new(),
            grid: world.grid.clone(),
            color,
        }
    }
}
