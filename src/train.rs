use rurel::{
    dqn::DQNAgentTrainer,
    mdp::{Agent, State},
    strategy::explore::RandomExploration,
};

use crate::world::{
    grid::{Grid, Pos, Tile},
    species::Species,
    World,
};
use terminate::FixedIterations;

mod terminate;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
        if arr[0] == 0.0 {
            Self::Move(arr[1] as i8, arr[2] as i8)
        } else if arr[0] == 0.5 {
            Self::Attack(arr[1] as i8, arr[2] as i8)
        } else if arr[0] == 1.0 {
            Self::BuildWall(arr[1] as i8, arr[2] as i8)
        } else {
            panic!("Unknown action {arr:?}")
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
    time_left: usize,
}

impl CreatureState {
    fn new(species: &Species, time_left: usize, index: usize) -> Self {
        Self {
            slice: species.get_view_slice(index),
            food: species.get_food(index),
            time_left,
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
            match self.slice[Pos(3, 3) + direction] {
                Tile::Empty => {
                    actions.push(CreatureAction::Move(direction.0, direction.1));
                    if self.food > 0 {
                        actions.push(CreatureAction::BuildWall(direction.0, direction.1));
                    }
                }
                Tile::Bush(true) | Tile::Creature { .. } | Tile::Wall { .. } => {
                    actions.push(CreatureAction::Attack(direction.0, direction.1));
                }
                _ => (),
            }
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
        vec.push(self.time_left as f32);

        vec.try_into().expect("too many tiles in grid")
    }
}

pub struct SpeciesAgent<'a> {
    state: CreatureState,
    species: &'a Species,
    time_left: usize,
    creature_index: usize,
    iters: usize,
}

impl<'a> SpeciesAgent<'a> {
    pub fn new(species: &'a Species) -> Self {
        Self {
            state: CreatureState::new(&species, 0, 0),
            species,
            time_left: 0,
            creature_index: 0,
            iters: 0,
        }
    }

    pub fn increment_index(&mut self) {
        self.creature_index += 1;
        self.state = CreatureState::new(&self.species, self.time_left, self.creature_index);
    }

    pub fn reset_index(&mut self) {
        self.creature_index = 0;
        self.state = CreatureState::new(&self.species, self.time_left, self.creature_index);
    }
}

impl<'a> Agent<CreatureState> for SpeciesAgent<'a> {
    fn current_state(&self) -> &CreatureState {
        &self.state
    }

    fn take_action(&mut self, action: &CreatureAction) {
        println!("take action #{}", self.creature_index);
        self.species.handle_action(*action, self.creature_index);
        if self.creature_index < self.iters - 1 {
            self.increment_index();
        }
    }
}

pub type SpeciesModel<'a> = (
    DQNAgentTrainer<CreatureState, 100, 3, 128>,
    SpeciesAgent<'a>,
    &'a Species,
);

impl World {
    pub fn train_moons(
        &mut self,
        num_moons: usize,
    ) -> Vec<DQNAgentTrainer<CreatureState, 100, 3, 128>> {
        let mut models: Vec<SpeciesModel> = Vec::new();
        for species in &self.species {
            models.push((
                DQNAgentTrainer::new(0.9, 1e-3),
                SpeciesAgent::new(&species),
                species,
            ));
        }

        for moon in 0..num_moons {
            for step in 0..self.config.moon_len {
                for (trainer, agent, species) in &mut models {
                    let iterations = species.members.borrow().len();
                    if iterations == 0 {
                        continue;
                    }

                    agent.time_left = self.config.moon_len - step;
                    agent.iters = iterations;
                    trainer.train(
                        agent,
                        &mut FixedIterations::new(iterations as u32),
                        &RandomExploration::new(),
                    );
                    agent.reset_index();
                }

                self.finish_step();
            }
            self.finish_moon();
            println!("Moon {moon}/{num_moons}")
        }

        models.into_iter().map(|(trainer, ..)| trainer).collect()
    }
}
