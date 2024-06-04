use std::{cell::RefCell, rc::Rc};

use raylib::color::Color;

use super::grid::{Grid, GridPos};
use crate::world::World;

pub struct SpeciesConfig {
    /// Color of creatures
    pub color: Color,
    /// Number of creatures in a pack
    pub num_creatures: usize,
    /// Number of packs
    pub num_packs: usize,
}

pub struct Species {
    members: Vec<GridPos>,
    grid: Rc<RefCell<Grid<40, 30>>>,
    color: Color,
}

impl Species {
    pub(super) fn new(world: &World, color: Color) -> Self {
        Self {
            members: Vec::new(),
            grid: world.grid.clone(),
            color,
        }
    }
}
