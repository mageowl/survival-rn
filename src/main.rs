use render::run_simulation;
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
    });

    world.add_species(SpeciesConfig {
        color: HEPT32::GREEN,
        num_creatures: 4,
        num_packs: 2,
    });
    world.add_species(SpeciesConfig {
        color: HEPT32::BLUE,
        num_creatures: 1,
        num_packs: 8,
    });

    world.train_moons(1);
    run_simulation(world);
}
