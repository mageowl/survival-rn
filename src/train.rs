use rurel::mdp::{Agent, State};

use crate::world::{
    grid::{Grid, Pos, Tile},
    species::Species,
    World,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CreatureAction {
    Move(i8, i8),
    Attack(i8, i8),
    BuildWall(i8, i8),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CreatureState {
    /// 7x7 grid centered on creature.
    slice: Grid<7, 7>,
    /// Amount of food that the creature has.
    food: usize,
    /// How much time is left until the moon ends. (and it has to eat)
    time_left: usize,
}

impl CreatureState {
    fn new(species: &Species, world: &World, index: usize) -> Self {
        Self {
            slice: species.get_view_slice(index),
            food: species.get_food(index),
            time_left: world.time_left,
        }
    }
}

impl State for CreatureState {
    type A = CreatureAction;

    fn reward(&self) -> f64 {
        (2 - self.food) as f64 * -(self.time_left as f64)
    }

    fn actions(&self) -> Vec<Self::A> {
        let mut actions = Vec::new();

        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Tile::Empty = self.slice[Pos(4, 4) + direction] {
                actions.push(CreatureAction::Move(direction.0, direction.1));
                actions.push(CreatureAction::BuildWall(direction.0, direction.1));
            }

            if let Tile::Bush(true) | Tile::Creature { .. } = self.slice[Pos(4, 4) + direction] {
                actions.push(CreatureAction::Attack(direction.0, direction.1));
            }
        }

        actions
    }
}

pub struct SpeciesAgent<'a> {
    state: CreatureState,
    species: &'a Species,
    world: &'a World,
    creature_index: usize,
}

impl<'a> SpeciesAgent<'a> {
    pub fn new(species: &'a Species, world: &'a World) -> Self {
        Self {
            state: CreatureState::new(&species, &world, 0),
            species,
            world,
            creature_index: 0,
        }
    }
}

impl<'a> Agent<CreatureState> for SpeciesAgent<'a> {
    fn current_state(&self) -> &CreatureState {
        &self.state
    }

    fn take_action(&mut self, action: &<CreatureState as State>::A) {
        self.species.handle_action(*action, self.creature_index);

        self.creature_index += 1;
        self.state = CreatureState::new(&self.species, &self.world, self.creature_index);
    }
}
