use raylib::{
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

pub struct Assets {
    pub agent: Texture2D,
}

fn load_texture(path: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    rl.load_texture_from_image(&thread, &Image::load_image(path).expect("Image not found."))
        .expect("Failed to load texture")
}

pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Assets {
    Assets {
        agent: load_texture("assets/agent.png", rl, thread),
    }
}
