use raylib::{
    camera::Camera2D,
    color::Color,
    consts::TraceLogLevel,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Vector2,
};

mod assets;

const WORLD_SIZE: (i32, i32) = (640, 480);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WORLD_SIZE.0 * 2, WORLD_SIZE.1 * 2)
        .title("Survival Sim")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let assets = assets::load(&mut rl, &thread);

    let camera = Camera2D {
        target: Vector2::zero(),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: 2.0,
    };

    while !rl.window_should_close() {
        // UPDATE //

        // DRAW //
        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::TAN);

        d.draw_texture(&assets.agent, 0, 0, Color::new(89, 205, 75, 255))
        // d.draw_rectangle(-5, -5, 10, 10, Color::WHITE)
    }
}
