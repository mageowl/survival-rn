use std::{cell::RefCell, rc::Rc};

use grid::{Grid, Pos, Tile};
use species::{Species, SpeciesConfig};

use crate::util::{GRID_HEIGHT, GRID_WIDTH};

pub mod grid;
pub mod species;

#[derive(Clone, Copy)]
pub struct SimConfig {
    /// Number of steps per moon.
    pub moon_len: usize,
    /// Number of bushes to spawn.
    pub num_food: usize,
    /// Chance that a given bush regrows in between moons.
    pub chance_regrow: f64,
    /// List of species to include in simulation
    pub species: &'static [SpeciesConfig],
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

        let mut s = Self {
            grid: Rc::new(RefCell::new(grid)),
            species: Vec::new(),
            time_left: config.moon_len,
            config,
        };

        for species in config.species {
            s.add_species(*species);
        }

        s
    }

    fn add_species(&mut self, config: SpeciesConfig) {
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

    pub fn finish_step(&self) {
        for species in &self.species {
            let mut creatures = species.members.borrow_mut();

            let mut indices = Vec::new();
            let mut clear_pos = Vec::new();
            for (i, creature) in creatures.iter().enumerate() {
                if let Tile::Creature { food, .. } = self.grid.borrow()[*creature] {
                    if food < 0 {
                        clear_pos.push(*creature);
                        indices.push(i);
                    }
                } else {
                    clear_pos.push(*creature);
                    indices.push(i);
                }
            }

            indices.reverse();
            for i in indices {
                creatures.remove(i);
            }

            for pos in clear_pos {
                self.grid.borrow_mut()[pos] = Tile::Empty;
            }
        }
    }

    pub fn finish_moon(&self) {
        let mut grid = self.grid.borrow_mut();
        for (y, row) in grid.arr().iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match *tile {
                    Tile::Bush(false) => {
                        if rand::random::<f64>() > self.config.chance_regrow {
                            grid[Pos(x, y)] = Tile::Bush(true);
                        }
                    }
                    Tile::Creature {
                        food,
                        species,
                        color,
                    } => {
                        grid[Pos(x, y)] = Tile::Creature {
                            species,
                            color,
                            food: food - 2,
                        };
                    }
                    _ => (),
                }
            }
        }

        drop(grid);
        self.finish_step()
    }
}
