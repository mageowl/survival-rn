use raylib::{
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

pub struct Assets {
    pub agent: Texture2D,
    pub bush: Texture2D,
    pub bush_berries: Texture2D,
    pub wall: Texture2D,
}

fn load_texture(path: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    rl.load_texture_from_image(&thread, &Image::load_image(path).expect("Image not found."))
        .expect("Failed to load texture")
}

pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Assets {
    Assets {
        agent: load_texture("assets/agent.png", rl, thread),
        bush: load_texture("assets/bush.png", rl, thread),
        bush_berries: load_texture("assets/bush_berries.png", rl, thread),
        wall: load_texture("assets/wall.png", rl, thread),
    }
}
