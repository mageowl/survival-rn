use std::time::Instant;

use rurel::{
    dqn::DQNAgentTrainer,
    mdp::{Agent, State},
    strategy::explore::RandomExploration,
};

use crate::world::{
    grid::{Grid, Pos, Tile},
    species::Species,
    SimConfig, World,
};
use terminate::FixedIterations;

pub mod terminate;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CreatureAction {
    Move(i8, i8),
    Attack(i8, i8),
    BuildWall(i8, i8),
    DoNothing,
}

impl CreatureAction {
    const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
}

impl Into<[f32; 4]> for CreatureAction {
    fn into(self) -> [f32; 4] {
        match self {
            CreatureAction::Move(x, y) => {
                Self::DIRECTIONS.map(|(x1, y1)| if x1 == x && y1 == y { 1.0 } else { 0.0 })
            }
            CreatureAction::Attack(x, y) => {
                Self::DIRECTIONS.map(|(x1, y1)| if x1 == x && y1 == y { 1.0 } else { 0.0 })
            }
            CreatureAction::BuildWall(_, _) => todo!(),
            CreatureAction::DoNothing => [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl From<[f32; 4]> for CreatureAction {
    fn from(arr: [f32; 4]) -> Self {
        let one_hot = arr.map(|v| v.clamp(0.0, 1.0));

        let mut largest_action = (0, 0);
        let mut largest_confidence = 0.0;
        for (i, confidence) in one_hot.into_iter().enumerate() {
            if confidence > largest_confidence {
                largest_action = Self::DIRECTIONS[i];
                largest_confidence = confidence;
            }
        }

        if largest_action == (0, 0) {
            CreatureAction::DoNothing
        } else {
            CreatureAction::Move(largest_action.0, largest_action.1)
        }
    }
}

pub trait OneHotEncodedAction {
    fn into_action(self, state: &CreatureState) -> CreatureAction;
}

impl OneHotEncodedAction for [f32; 4] {
    fn into_action(self, state: &CreatureState) -> CreatureAction {
        let one_hot = self.map(|v| v.clamp(0.0, 1.0));

        let mut largest_action = (0, 0);
        let mut largest_confidence = 0.0;
        for (i, confidence) in one_hot.into_iter().enumerate() {
            if confidence > largest_confidence {
                largest_action = CreatureAction::DIRECTIONS[i];
                largest_confidence = confidence;
            }
        }

        if largest_action == (0, 0) {
            CreatureAction::DoNothing
        } else {
            match state.slice[Pos(3, 3) + largest_action] {
                Tile::Empty => CreatureAction::Move(largest_action.0, largest_action.1),
                Tile::OutOfBounds => CreatureAction::DoNothing,
                _ => CreatureAction::Attack(largest_action.0, largest_action.1),
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CreatureState {
    /// 7x7 grid centered on creature.
    slice: Grid<7, 7>,
    /// Amount of food that the creature has.
    food: isize,
    /// How much time is left until the moon ends. (and it has to eat)
    time: usize,
}

impl CreatureState {
    pub fn new(species: &Species, time: usize, index: usize) -> Self {
        Self {
            slice: species.get_view_slice(index),
            food: species.get_food(index),
            time,
        }
    }
}

impl State for CreatureState {
    type A = CreatureAction;

    fn reward(&self) -> f64 {
        (2 - self.food) as f64 * -(self.time as f64) * 1000.0
    }

    fn actions(&self) -> Vec<Self::A> {
        let mut actions = Vec::new();

        for direction in CreatureAction::DIRECTIONS {
            match self.slice[Pos(3, 3) + direction] {
                Tile::Empty => {
                    actions.push(CreatureAction::Move(direction.0, direction.1));
                    if self.food > 0 {
                        // actions.push(CreatureAction::BuildWall(direction.0, direction.1));
                    }
                }
                Tile::Bush(true) | Tile::Creature { .. } | Tile::Wall { .. } => {
                    actions.push(CreatureAction::Attack(direction.0, direction.1));
                }
                _ => (),
            }
        }

        if actions.len() == 0 {
            actions.push(CreatureAction::DoNothing);
        }

        actions
    }
}

impl Into<[f32; 100]> for CreatureState {
    fn into(self) -> [f32; 100] {
        let mut vec = Vec::new();

        for tile in self.slice.arr().iter().flatten() {
            vec.extend_from_slice(&Into::<[f32; 2]>::into(*tile));
        }

        vec.push(self.food as f32);
        vec.push(self.time as f32);

        vec.try_into().expect("too many tiles in grid")
    }
}

pub struct SpeciesAgent<'a> {
    state: CreatureState,
    species: &'a Species,
    pub time: usize,
    creature_index: usize,
    pub iters: usize,
}

impl<'a> SpeciesAgent<'a> {
    pub fn new(species: &'a Species) -> Self {
        Self {
            state: CreatureState::new(&species, 0, 0),
            species,
            time: 0,
            creature_index: 0,
            iters: 0,
        }
    }

    pub fn increment_index(&mut self) {
        self.creature_index += 1;
        self.state = CreatureState::new(&self.species, self.time, self.creature_index);
    }

    pub fn reset_index(&mut self) {
        self.creature_index = 0;
        self.state = CreatureState::new(&self.species, self.time, self.creature_index);
    }
}

impl<'a> Agent<CreatureState> for SpeciesAgent<'a> {
    fn current_state(&self) -> &CreatureState {
        &self.state
    }

    fn take_action(&mut self, action: &CreatureAction) {
        self.species.handle_action(*action, self.creature_index);
        if self.creature_index < self.iters - 1 {
            self.increment_index();
        }
    }
}

impl SimConfig {
    pub fn create_dqn_models(&self) -> Vec<SpeciesModel> {
        let mut models = Vec::new();

        for _ in self.species {
            models.push(Default::default());
        }

        models
    }
}

pub type SpeciesModel = DQNAgentTrainer<CreatureState, 100, 4, 128>;

pub fn train_moons(world: &mut World, models: &mut Vec<SpeciesModel>, num_moons: usize) {
    let mut species_data = Vec::new();
    for (species, model) in world.species.iter().zip(models.iter_mut()) {
        species_data.push((model, SpeciesAgent::new(&species), species));
    }

    for moon in 0..num_moons {
        for step in 0..world.config.moon_len {
            for (trainer, agent, species) in &mut species_data {
                let iterations = species.members.borrow().len();
                if iterations == 0 {
                    continue;
                }

                agent.reset_index();
                agent.time = step;
                agent.iters = iterations;

                trainer.train(
                    agent,
                    &mut FixedIterations::new(iterations as u32),
                    &RandomExploration::new(),
                );
            }

            world.finish_step();
        }

        world.finish_moon();
        println!("  Moon {}/{num_moons}", moon + 1);

        if !world.species.iter().any(|s| s.members.borrow().len() > 0) {
            println!("  Finished due to extinction.");
            break;
        }
    }
}

pub fn train_iters(config: SimConfig, num_iters: usize, num_moons: usize) -> Vec<SpeciesModel> {
    let mut models = config.create_dqn_models();

    for i in 0..num_iters {
        let mut world = World::new(config);
        let now = Instant::now();

        train_moons(&mut world, &mut models, num_moons);

        println!(
            "# Epoch {}/{num_iters} completed in {} seconds.\n",
            i + 1,
            now.elapsed().as_secs() as f64
                + (now.elapsed().subsec_millis() as f64 / 100.0).round() / 10.0,
        )
    }

    models
}
