use render::run_simulation;
use train::train_moons;
use util::HEPT32;
use world::{species::SpeciesConfig, SimConfig, World};

mod render;
mod train;
mod util;
mod world;

fn main() {
    let mut world = World::new(SimConfig {
        moon_len: 20,
        num_food: 20,
        chance_regrow: 0.5,
        species: &[
            SpeciesConfig {
                color: HEPT32::RED,
                num_creatures: 4,
                num_packs: 2,
            },
            SpeciesConfig {
                color: HEPT32::BLUE,
                num_creatures: 1,
                num_packs: 8,
            },
        ],
    });

    let models = train_moons(&mut world, 1);
    run_simulation(&mut world, models);
}
