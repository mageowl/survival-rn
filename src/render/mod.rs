use raylib::{
    camera::Camera2D,
    color::Color,
    drawing::{RaylibDraw, RaylibMode2DExt},
    ffi::TraceLogLevel,
    math::Vector2,
};
use rurel::{mdp::State, strategy::explore::RandomExploration};

use crate::{
    train::{
        terminate::FixedIterations, CreatureState, OneHotEncodedAction, SpeciesAgent, SpeciesModel,
    },
    util::{GRID_HEIGHT, GRID_WIDTH, TILE_SIZE},
    world::World,
};

pub mod assets;

const ZOOM: f32 = 2.0;
const VIEW_SIZE: (i32, i32) = (
    GRID_WIDTH as i32 * TILE_SIZE * ZOOM as i32,
    GRID_HEIGHT as i32 * TILE_SIZE * ZOOM as i32,
);

pub fn run_simulation(world: &mut World, mut models: Vec<SpeciesModel>) {
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

    let mut species_data = Vec::new();
    for (species, model) in world.species.iter().zip(models.iter_mut()) {
        species_data.push((model, SpeciesAgent::new(&species), species));
    }

    let mut step_timer = 0.0;
    let mut time_left = world.config.moon_len;
    while !rl.window_should_close() {
        // UPDATE //
        step_timer += rl.get_frame_time();
        if step_timer >= 1.0 {
            step_timer = 0.0;
            time_left -= 1;

            for (trainer, agent, species) in &mut species_data {
                let iterations = species.members.borrow().len();
                if iterations == 0 {
                    continue;
                }

                agent.reset_index();
                agent.time = world.config.moon_len - time_left;
                agent.iters = iterations;

                trainer.train(
                    agent,
                    &mut FixedIterations::new(iterations as u32),
                    &RandomExploration::new(),
                );
            }

            world.finish_step();

            if time_left <= 0 {
                time_left = world.config.moon_len;
                world.finish_moon();
            }
        }

        // DRAW //
        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::TAN);

        world.grid.borrow().render(&mut d, &assets);
    }
}
