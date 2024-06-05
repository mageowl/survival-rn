use rurel::{
    dqn::DQNAgentTrainer,
    mdp::{Agent, State},
};

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

impl Into<[f32; 3]> for CreatureAction {
    fn into(self) -> [f32; 3] {
        match self {
            CreatureAction::Move(x, y) => [0.0, x as f32, y as f32],
            CreatureAction::Attack(x, y) => [0.5, x as f32, y as f32],
            CreatureAction::BuildWall(x, y) => [1.0, x as f32, y as f32],
        }
    }
}

impl From<[f32; 3]> for CreatureAction {
    fn from(arr: [f32; 3]) -> Self {
        match arr[0] {
            0.0 => Self::Move(arr[1] as i8, arr[2] as i8),
            0.5 => Self::Attack(arr[1] as i8, arr[2] as i8),
            1.0 => Self::BuildWall(arr[1] as i8, arr[2] as i8),
            _ => panic!("Unknown action {arr:?}"),
        }
    }
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

impl Into<[f32; 100]> for CreatureState {
    fn into(self) -> [f32; 100] {
        let mut vec = Vec::new();

        for (i, tile) in self.slice.arr().iter().flatten().enumerate() {
            vec.extend_from_slice(&Into::<[f32; 2]>::into(*tile));
        }

        vec.push(self.food as f32);
        vec.push(self.time_left as f32);

        vec.try_into().expect("too many tiles in grid")
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

pub fn train_species(&mut world: &mut World, num_moons: usize) {
    let mut trainers = Vec::new();
    let mut agents = Vec::new();

    for species in world.species {
        trainers.push(DQNAgentTrainer::<CreatureState, 100, 3, 128>::new(
            0.9, 1e-3,
        ));
        agents.push(SpeciesAgent::new(&species, &world));
    }
}
