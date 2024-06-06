use std::{cell::RefCell, rc::Rc};

use grid::{Grid, Pos, Tile};
use species::{Species, SpeciesConfig};

use crate::util::{GRID_HEIGHT, GRID_WIDTH};

pub mod grid;
pub mod species;

pub struct SimConfig {
    /// Number of steps per moon.
    pub moon_len: usize,
    /// Number of bushes to spawn.
    pub num_food: usize,
    /// Chance that a given bush regrows in between moons.
    pub chance_regrow: f64,
}

pub struct World {
    pub grid: Rc<RefCell<Grid<GRID_WIDTH, GRID_HEIGHT>>>,
    pub species: Vec<Species>,
    pub time_left: usize,
    pub config: SimConfig,
}

impl World {
    pub fn new(config: SimConfig) -> Self {
        let mut grid = Grid::empty();

        for _i in 0..config.num_food {
            let mut pos;

            loop {
                pos = Pos(
                    (rand::random::<f64>() * GRID_WIDTH as f64).floor() as usize,
                    (rand::random::<f64>() * GRID_HEIGHT as f64).floor() as usize,
                );

                if let Tile::Empty = grid[pos] {
                    break;
                }
            }

            grid[pos] = Tile::Bush(true);
        }

        Self {
            grid: Rc::new(RefCell::new(grid)),
            species: Vec::new(),
            time_left: config.moon_len,
            config,
        }
    }

    pub fn add_species(&mut self, config: SpeciesConfig) {
        let species = Species::new(self.species.len(), &self, config.color);

        for _ in 0..config.num_packs {
            let mut pos;

            loop {
                pos = Pos(
                    (rand::random::<f64>() * GRID_WIDTH as f64).floor() as usize,
                    (rand::random::<f64>() * GRID_HEIGHT as f64).floor() as usize,
                );

                if pos.0 <= GRID_WIDTH - 4 && pos.1 <= GRID_HEIGHT - 4 {
                    if let Tile::Empty = self.grid.borrow()[pos] {
                        break;
                    }
                }
            }

            for _ in 0..config.num_creatures {
                let mut offset;

                loop {
                    offset = Pos(
                        (rand::random::<f64>() * 5.0).floor() as usize,
                        (rand::random::<f64>() * 5.0).floor() as usize,
                    );

                    if let Tile::Empty = self.grid.borrow()[pos + offset] {
                        break;
                    }
                }

                self.grid.borrow_mut()[pos + offset] = Tile::Creature {
                    species: species.id,
                    color: species.color,
                    food: 0,
                };
                species.members.borrow_mut().push(pos + offset);
            }
        }

        self.species.push(species);
    }

    pub fn check_dead_creatures(&self) {
        for species in &self.species {
            let mut creatures = species.members.borrow_mut();

            let mut indices = Vec::new();
            for (i, creature) in creatures.iter().enumerate() {
                if let Tile::Creature { food, .. } = self.grid.borrow()[*creature] {
                    if food < 0 {
                        indices.push(i);
                    }
                } else {
                    indices.push(i);
                }
            }

            for i in indices {
                creatures.remove(i);
            }
        }
    }
}
