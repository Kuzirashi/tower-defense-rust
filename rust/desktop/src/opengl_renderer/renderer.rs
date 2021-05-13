use game::core::{GameRenderer, Point, Sprite, SpriteType};
use game::game::Game;
use game::{
    core::config::{MAP_TILES_AMOUNT_Y, SCREEN_HEIGHT, SCREEN_WIDTH, TARGET_FPS, TILE_PIXEL_SIZE},
    game::MouseState,
};
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::GLProfile;
use sdl2::video::{Window, WindowContext};
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
};
use sdl2::{pixels::Color, render::BlendMode};
use std::path::Path;
use std::time::SystemTime;

const SHOW_FPS_COUNTER: bool = false;

pub struct CachedTexture<'a> {
    texture_path: String,
    texture: Option<Texture<'a>>,
}

pub struct TextureCache<'a> {
    empty_texture: CachedTexture<'a>,
    cached_textures: Vec<CachedTexture<'a>>,
}

impl<'a> TextureCache<'a> {
    pub fn new() -> TextureCache<'a> {
        TextureCache {
            cached_textures: vec![],
            empty_texture: CachedTexture {
                texture_path: String::from(""),
                texture: None,
            },
        }
    }

    fn get_texture(&self, texture_path: &String) -> &CachedTexture<'a> {
        let mut iter = self.cached_textures.iter();

        let cached_texture = iter.find(|&x| &x.texture_path == texture_path);

        match cached_texture {
            Some(_) => return cached_texture.as_ref().unwrap(),
            _ => return &self.empty_texture,
        }
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        texture_path: &String,
    ) -> Result<&CachedTexture, String> {
        let existing_texture = self.get_texture(texture_path);

        if existing_texture.texture_path != self.empty_texture.texture_path {
            // return Ok(existing_texture);
            return Err(String::from("Texture already loaded."));
        } else {
            // println!("Attempt load texture: {}", texture_path);

            match texture_creator.load_texture(Path::new(&texture_path)) {
                Ok(texture) => {
                    let new_texture = CachedTexture {
                        texture_path: texture_path.to_owned(),
                        texture: Some(texture),
                    };
                    self.cached_textures.push(new_texture);
                    let mut iter = self.cached_textures.iter();
                    let cached_texture = iter.find(|&x| &x.texture_path == texture_path).unwrap();
                    return Ok(cached_texture);
                }
                Err(_) => {
                    println!("Texture load error");
                    return Err(String::from("Texture load error."));
                }
            }
        }
    }
}

pub struct OpenGLRenderer<'a> {
    pub name: String,
    pub canvas: &'a mut Canvas<Window>,

    texture_location: &'a str,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_cache: TextureCache<'a>,
    ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl<'a> OpenGLRenderer<'a> {
    pub fn new(
        name: String,
        texture_location: &'a str,
        canvas: &'a mut Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>, // texture_cache: &'a mut TextureCache<'a>,
    ) -> Result<OpenGLRenderer<'a>, String> {
        let ttf_context = sdl2::ttf::init().unwrap();

        let renderer = OpenGLRenderer {
            name,
            canvas,
            texture_location,
            texture_creator,
            texture_cache: TextureCache::new(),
            ttf_context,
        };

        Ok(renderer)
    }

    fn get_texture_path(&self, texture_path: &String) -> String {
        let mut first = self.texture_location.to_owned();
        first.push_str(texture_path);

        first.clone()
    }
}

impl GameRenderer for OpenGLRenderer<'_> {
    fn draw(&mut self, sprites: &Vec<Sprite>) {
        let bg_color = Color::RGB(5, 5, 5);
        self.canvas.set_draw_color(bg_color);

        self.canvas.clear();

        let font_path = Path::new("/home/kuzi/projects/tower-rust/rust/assets/fonts/arial.ttf");
        let font8 = self.ttf_context.load_font(font_path, 8).unwrap();
        let font16 = self.ttf_context.load_font(font_path, 16).unwrap();
        let font32 = self.ttf_context.load_font(font_path, 32).unwrap();

        for sprite in sprites {
            match sprite.sprite_type {
                SpriteType::Text => {
                    let font = match sprite.font_size {
                        8 => &font8,
                        16 => &font16,
                        32 => &font32,
                        _ => &font16,
                    };

                    let surface = font
                        .render(&sprite.text)
                        .blended(Color::RGBA(255, 255, 255, 255))
                        .map_err(|e| e.to_string())
                        .unwrap();
                    let texture = self
                        .texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())
                        .unwrap();

                    let TextureQuery { width, height, .. } = texture.query();

                    self.canvas.set_blend_mode(BlendMode::Blend);

                    self.canvas.copy(
                        &texture,
                        None,
                        Rect::new(sprite.position.x, sprite.position.y, width, height),
                    );
                }
                SpriteType::Image => {
                    let texture_path = self.get_texture_path(&sprite.texture_path);

                    if sprite.visible {
                        self.texture_cache
                            .load_texture(&self.texture_creator, &texture_path);

                        let texture = self
                            .texture_cache
                            .get_texture(&texture_path)
                            .texture
                            .as_ref()
                            .unwrap();

                        self.canvas.copy_ex(
                            &texture,
                            None,
                            Rect::new(
                                sprite.position.x,
                                sprite.position.y,
                                sprite.width,
                                sprite.height,
                            ),
                            sprite.rotation,
                            None,
                            false,
                            false,
                        );
                    }
                }
                SpriteType::Rect => {
                    self.canvas.set_draw_color(Color::RGBA(
                        sprite.color.r,
                        sprite.color.g,
                        sprite.color.b,
                        sprite.color.a,
                    ));

                    self.canvas.fill_rect(Rect::new(
                        sprite.position.x,
                        sprite.position.y,
                        sprite.width,
                        sprite.height,
                    ));
                }
            }
        }

        self.canvas.present();
    }
}

pub struct OpenGLGame {
    pub canvas: Canvas<Window>,
    pub sdl_context: sdl2::Sdl,

    game: Game,
    event_pump: sdl2::EventPump,
}

impl OpenGLGame {
    pub fn new() -> OpenGLGame {
        let sdl_context = sdl2::init().or_else(|error| Err(error)).unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 1);

        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
        let event_pump = sdl_context.event_pump().unwrap();

        let game = Game::new();

        let window = video_subsystem
            .window(
                "rust-sdl2 demo: Video",
                SCREEN_HEIGHT as u32,
                SCREEN_WIDTH as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        OpenGLGame {
            canvas,
            sdl_context,
            event_pump,
            game,
        }
    }

    pub fn start_update_loop(&mut self) -> Result<(), String> {
        self.game.start_round();

        let texture_creator = self.canvas.texture_creator();

        let mut renderer = OpenGLRenderer::new(
            String::from("Tower Defense SDL2"),
            "/home/kuzi/projects/tower-rust/rust",
            &mut self.canvas,
            &texture_creator,
        )?;

        let start = SystemTime::now();

        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(TARGET_FPS);

        let mut rendered_frames: u128 = 0;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running Ok(()),
                    _ => {}
                }
            }

            let mouse_state = self.event_pump.mouse_state();

            let now = SystemTime::now();
            let since_start = now
                .duration_since(start)
                .expect("Time went backwards")
                .as_millis();

            self.game.update(
                since_start as f64,
                MouseState::new(
                    mouse_state.left(),
                    Point::new(mouse_state.x(), mouse_state.y()),
                ),
            );
            let mut sprites = self.game.get_sprites();

            if SHOW_FPS_COUNTER {
                let mut fps_msg = "FPS (calls): ".to_owned();
                let fps = rendered_frames
                    .checked_div(since_start / 1000)
                    .unwrap_or(0)
                    .to_string();
                fps_msg.push_str(&fps);

                sprites.push(Sprite::create_text(
                    &fps_msg,
                    Point::new(10, TILE_PIXEL_SIZE * MAP_TILES_AMOUNT_Y as i32),
                    16,
                ));
            }

            renderer.draw(&sprites);

            rendered_frames += 1;

            fps_manager.delay();
        }
    }
}
