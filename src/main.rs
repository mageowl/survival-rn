use render::open_window;
use train::train_species;
use util::HEPT32;
use world::{species::SpeciesConfig, SimConfig, World};

mod render;
mod train;
mod util;
mod world;

fn main() {
    let mut world = World::new(SimConfig {
        moon_len: 1,
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

    train_species(&mut world, 1);
    open_window(world);
}
