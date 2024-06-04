use std::{cell::RefCell, rc::Rc};

use grid::{Grid, Tile};
use species::{Species, SpeciesConfig};

mod grid;
mod species;

pub struct SimConfig {
    /// Number of moons to simulate.
    pub num_moons: usize,
    /// Number of bushes to spawn.
    pub num_food: usize,
    /// Chance that a given bush regrows in between moons.
    pub chance_regrow: f64,
}

pub struct World {
    pub grid: Rc<RefCell<Grid<40, 30>>>,
    pub species: Vec<Species>,
}

impl World {
    pub fn new(config: SimConfig) -> Self {
        let mut grid = Grid::empty();

        for _i in 0..config.num_food {
            let mut pos: (usize, usize) = (
                (rand::random::<f64>() * 40.0).floor() as usize,
                (rand::random::<f64>() * 30.0).floor() as usize,
            );

            while let Tile::Bush(_) = grid.at(pos) {
                pos = (
                    (rand::random::<f32>() * 40.0).floor() as usize,
                    (rand::random::<f32>() * 30.0).floor() as usize,
                );
            }

            grid.set(pos, Tile::Bush(true))
        }

        Self {
            grid: Rc::new(RefCell::new(grid)),
            species: Vec::new(),
        }
    }

    pub fn add_species(&mut self, config: SpeciesConfig) {
        self.species.push(Species::new(&self, config.color));
    }
}