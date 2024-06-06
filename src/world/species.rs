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
    pub members: RefCell<Vec<Pos>>,
    pub color: Color,
    grid: Rc<RefCell<Grid<GRID_WIDTH, GRID_HEIGHT>>>,
}

impl Species {
    pub fn new(id: usize, world: &World, color: Color) -> Self {
        Self {
            id,
            members: RefCell::new(Vec::new()),
            grid: world.grid.clone(),
            color,
        }
    }

    pub fn get_view_slice(&self, index: usize) -> Grid<7, 7> {
        let i_pos: IPos = Into::<IPos>::into(self.members.borrow()[index]) - IPos(3, 3);

        if let Ok(pos) = i_pos.try_into() {
            self.grid.borrow().slice(pos)
        } else {
            self.grid.borrow().i_slice(i_pos)
        }
        .into()
    }

    pub fn get_food(&self, index: usize) -> usize {
        if let Tile::Creature { food, .. } = self.grid.borrow()[self.members.borrow()[index]] {
            food
        } else {
            println!("{:?}", self.grid.borrow()[self.members.borrow()[index]]);
            panic!(
                "Expected creature at position {}. (Trying to access amount of food)",
                self.members.borrow()[index]
            );
        }
    }

    pub fn handle_action(&self, action: CreatureAction, index: usize) {
        let mut grid = self.grid.borrow_mut();
        let mut members = self.members.borrow_mut();

        match action {
            CreatureAction::Move(x, y) => {
                grid[members[index] + (x, y)] = grid[members[index]];
                members[index] = members[index] + (x, y);
            }
            CreatureAction::Attack(x, y) => {
                grid[members[index] + (x, y)] = match grid[members[index] + (x, y)] {
                    Tile::Empty => Tile::Empty,
                    Tile::OutOfBounds => Tile::Empty,
                    Tile::Bush(true) => Tile::Bush(false),
                    Tile::Bush(false) => Tile::Empty,
                    Tile::Wall { .. } => Tile::Empty,
                    Tile::Creature {
                        species,
                        color,
                        food,
                    } => {
                        println!("{}", food);
                        if food > 0 {
                            Tile::Creature {
                                species,
                                color,
                                food: food - 1,
                            }
                        } else {
                            Tile::Empty
                        }
                    }
                };
                grid[members[index]] = match grid[members[index]] {
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
                        members[index]
                    ),
                }
            }
            CreatureAction::BuildWall(x, y) => {
                grid[members[index] + (x, y)] = Tile::Wall {
                    species: match grid[members[0]] {
                        Tile::Creature { species, .. } => species,
                        _ => panic!(
                            "Expected creature at position {}. (Trying to get species)",
                            members[0]
                        ),
                    },
                    color: self.color,
                }
            }
        }
    }
}
