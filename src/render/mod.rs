use raylib::{
    camera::Camera2D,
    color::Color,
    drawing::{RaylibDraw, RaylibMode2DExt},
    ffi::TraceLogLevel,
    math::Vector2,
};

use crate::{
    util::{GRID_HEIGHT, GRID_WIDTH, TILE_SIZE},
    world::World,
};

pub mod assets;

const ZOOM: f32 = 2.0;
const VIEW_SIZE: (i32, i32) = (
    GRID_WIDTH as i32 * TILE_SIZE * ZOOM as i32,
    GRID_HEIGHT as i32 * TILE_SIZE * ZOOM as i32,
);

pub fn run_simulation(world: World) {
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

    let mut time = 0.0;
    while !rl.window_should_close() {
        // UPDATE //
        time += rl.get_frame_time();
        if time >= 1.0 {
            time = 0.0;
        }

        // DRAW //
        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::TAN);

        world.grid.borrow().render(&mut d, &assets);
    }
}
