use raylib::{
    camera::Camera2D,
    color::Color,
    consts::TraceLogLevel,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Vector2,
};
use world::{SimConfig, World};

mod assets;
mod world;

const ZOOM: f32 = 2.0;
const VIEW_SIZE: (i32, i32) = (40 * 16 * ZOOM as i32, 30 * 16 * ZOOM as i32);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(VIEW_SIZE.0, VIEW_SIZE.1)
        .title("Survival Sim")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let assets = assets::load(&mut rl, &thread);
    let world = World::new(SimConfig {
        num_moons: 10,
        num_food: 60,
        chance_regrow: 0.5,
    });

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
        // d.draw_rectangle(-5, -5, 10, 10, Color::WHITE)
    }
}
