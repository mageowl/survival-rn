use raylib::{
    camera::Camera2D,
    color::Color,
    consts::TraceLogLevel,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Vector2,
};
use util::{GRID_HEIGHT, GRID_WIDTH, HEPT32, TILE_SIZE};
use world::{species::SpeciesConfig, SimConfig, World};

mod assets;
mod train;
mod util;
mod world;

const ZOOM: f32 = 2.0;
const VIEW_SIZE: (i32, i32) = (
    GRID_WIDTH as i32 * TILE_SIZE * ZOOM as i32,
    GRID_HEIGHT as i32 * TILE_SIZE * ZOOM as i32,
);

fn main() {
    let mut world = World::new(SimConfig {
        num_moons: 10,
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

    let (mut rl, thread) = raylib::init()
        .size(VIEW_SIZE.0, VIEW_SIZE.1)
        .title("Survival Sim")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let assets = assets::load(&mut rl, &thread);
    let camera = Camera2D {
        target: Vector2::zero(),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: ZOOM,
    };

    while !rl.window_should_close() {
        // UPDATE //

        // DRAW //
        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::TAN);
        world.grid.borrow().render(&mut d, &assets);
    }
}
