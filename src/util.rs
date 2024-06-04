use raylib::color::Color;

pub const GRID_WIDTH: usize = 30;
pub const GRID_HEIGHT: usize = 20;

pub const TILE_SIZE: i32 = 16;

pub struct HEPT32;

#[allow(unused)]
impl HEPT32 {
    pub const BLACK: Color = Color::BLACK;
    pub const TAN: Color = Color::new(251, 190, 130, 255);

    pub const BLUE: Color = Color::new(94, 233, 233, 255);
    pub const GREEN: Color = Color::new(71, 246, 65, 255);
    pub const RED: Color = Color::new(249, 78, 109, 255);
    pub const YELLOW: Color = Color::new(236, 171, 17, 255);
}
