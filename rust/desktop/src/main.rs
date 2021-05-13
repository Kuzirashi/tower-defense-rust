mod opengl_renderer;
use opengl_renderer::renderer::{OpenGLGame};

pub fn main() -> Result<(), String> {
    let mut game = OpenGLGame::new();

    game.start_update_loop();

    Ok(())
}
