use std::io;

use render::run_simulation;
use train::{train_iters, train_moons};
use util::HEPT32;
use world::{species::SpeciesConfig, SimConfig, World};

mod render;
mod train;
mod util;
mod world;

const CONFIG: SimConfig = SimConfig {
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
};

fn main() {
    let models = train_iters(CONFIG, 1, 10);

    println!("Press ENTER to start simulation.");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input.");

    run_simulation(&mut World::new(CONFIG), models);
}
