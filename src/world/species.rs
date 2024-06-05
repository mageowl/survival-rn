use std::{cell::RefCell, rc::Rc};

use raylib::color::Color;

use super::grid::{Grid, IPos, Pos, Tile};
use crate::{
    train::CreatureAction,
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

    pub fn get_view_slice(&self, index: usize) -> Grid<7, 7> {
        let i_pos: IPos = Into::<IPos>::into(self.members[index]) - IPos(3, 3);

        if let Ok(pos) = i_pos.try_into() {
            self.grid.borrow().slice(pos)
        } else {
            self.grid.borrow().i_slice(i_pos)
        }
        .into()
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

    pub fn get_species_idx(&self) -> usize {
        match self.grid.borrow()[self.members[0]] {
            Tile::Creature { species, .. } => species,
            _ => panic!(
                "Expected creature at position {}. (Trying to get species)",
                self.members[0]
            ),
        }
    }

    pub fn handle_action(&self, action: CreatureAction, index: usize) {
        match action {
            CreatureAction::Move(x, y) => {
                self.grid.borrow_mut()[self.members[index] + (x, y)] =
                    self.grid.borrow()[self.members[index]];
            }
            CreatureAction::Attack(x, y) => {
                self.grid.borrow_mut()[self.members[index] + (x, y)] =
                    match self.grid.borrow()[self.members[index] + (x, y)] {
                        Tile::Empty => Tile::Empty,
                        Tile::OutOfBounds => Tile::Empty,
                        Tile::Bush(true) => Tile::Bush(false),
                        Tile::Bush(false) => Tile::Empty,
                        Tile::Wall { .. } => Tile::Empty,
                        Tile::Creature {
                            species,
                            color,
                            food,
                        } => Tile::Creature {
                            species,
                            color,
                            food: food - 1,
                        },
                    };
                self.grid.borrow_mut()[self.members[index]] =
                    match self.grid.borrow()[self.members[index]] {
                        Tile::Creature {
                            species,
                            color,
                            food,
                        } => Tile::Creature {
                            species,
                            color,
                            food: food + 1,
                        },
                        _ => panic!(
                            "Expected creature at position {}. (Trying to give food from attack)",
                            self.members[index]
                        ),
                    }
            }
            CreatureAction::BuildWall(x, y) => {
                self.grid.borrow_mut()[self.members[index] + (x, y)] = Tile::Wall {
                    species: self.get_species_idx(),
                    color: self.color,
                }
            }
        }
    }
}
