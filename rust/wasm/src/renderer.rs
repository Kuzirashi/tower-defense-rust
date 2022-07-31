use game::core::{GameRenderer, Sprite, SpriteType};

use futures::task::{Context, Poll};
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct CachedTexture {
    texture_path: String,
    texture: Option<HtmlImageElement>,
}

pub struct TextureCache {
    empty_texture: CachedTexture,
    cached_textures: Vec<CachedTexture>,
}

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(Cell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let image = self.image.take().unwrap();
                let failed = self.load_failed.get();

                if failed {
                    Poll::Ready(Err(()))
                } else {
                    Poll::Ready(Ok(image))
                }
            }
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                Poll::Pending
            }
            _ => Poll::Ready(Err(())),
        }
    }
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            cached_textures: vec![],
            empty_texture: CachedTexture {
                texture_path: String::from(""),
                texture: None,
            },
        }
    }

    fn get_texture(&self, texture_path: &str) -> &CachedTexture {
        let mut iter = self.cached_textures.iter();

        let cached_texture = iter.find(|&x| &x.texture_path == texture_path);

        match cached_texture {
            Some(_cached_texture) => _cached_texture,
            _ => return &self.empty_texture,
        }
    }

    pub async fn load_texture(&mut self, texture_path: &str) -> Result<&CachedTexture, String> {
        let existing_texture = self.get_texture(texture_path);

        if existing_texture.texture_path != self.empty_texture.texture_path {
            // Ok("Existing".to_string())
            // Ok(existing_texture)
            Err(String::from("Texture already loaded."))
        } else {
            // log("Attempt load texture:");
            // log(texture_path);

            let texture_path = texture_path.clone();
            let image = ImageFuture::new(&texture_path).await;

            // log("After await");

            match image {
                Ok(_image) => {
                    // log("In block");

                    let new_texture = CachedTexture {
                        texture_path: texture_path.to_owned(),
                        texture: Some(_image),
                    };

                    self.cached_textures.push(new_texture);

                    let mut iter = self.cached_textures.iter();
                    let cached_texture = iter.find(|&x| &x.texture_path == texture_path);

                    match cached_texture {
                        Some(_cached_texture) => Ok(_cached_texture),
                        None => {
                            unsafe {
                                error("No cached texture but it should be there");
                            };
                            Err("No cached texture in set after saving it".to_string())
                        }
                    }
                }
                Err(_) => {
                    let mut error_msg = "Can not load texture in TextureCache: ".to_owned();
                    error_msg.push_str(texture_path);
                    unsafe {
                        error(&error_msg);
                    };

                    Err("Error.".to_string())
                }
            }

            // log("xyz");
            // let new_texture = CachedTexture {
            //     texture_path: texture_path.to_owned(),
            //     texture: Some(image),
            // };
            // // self.cached_textures.push(new_texture);
            // // self.texture = Some(texture);
            // log("Texture loaded");

            // let mut iter = self.cached_textures.iter();
            // let cached_texture = iter.find(|&x| &x.texture_path == texture_path).unwrap();

            // Ok(cached_texture)

            // Ok(new_texture)

            // match texture_creator.load_texture(Path::new(&texture_path)) {
            //     Ok(texture) => {
            //         let new_texture = CachedTexture {
            //             texture_path: texture_path.to_owned(),
            //             texture: Some(texture),
            //         };
            //         self.cached_textures.push(new_texture);
            //         // self.texture = Some(texture);
            //         println!("Texture loaded");
            //         // return Ok(&newTexture);
            //         let mut iter = self.cached_textures.iter();
            //         let cached_texture = iter.find(|&x| &x.texture_path == texture_path).unwrap();
            //         return Ok(cached_texture);
            //     }
            //     Err(_) => {
            //         println!("Texture load error");
            //         return Err(String::from("Texture load error."));
            //     }
            // }
        }
    }
}

pub struct BrowserRenderer {
    canvas: Rc<CanvasRenderingContext2d>, // pub name: String,
    // pub canvas: &'a mut Canvas<Window>,

    // texture_creator: &'a TextureCreator<WindowContext>,
    // texture_cache: TextureCache<'a>,
    // texture_cache: &'a mut TextureCache<'a>,
    texture_cache: TextureCache,
}

impl BrowserRenderer {
    pub fn new(canvas: Rc<CanvasRenderingContext2d>) -> BrowserRenderer {
        let texture_cache = TextureCache::new();

        BrowserRenderer {
            canvas,
            texture_cache,
        }
    }
}

impl BrowserRenderer {
    pub async fn load_assets(&mut self) -> Result<String, String> {
        let assets: Vec<&str> = vec![
            "/assets/tiles/ground_1.png",
            "/assets/tiles/ice_1.png",
            "/assets/creatures/creeper/right_0.png",
            "/assets/creatures/creeper/right_1.png",
            "/assets/creatures/creeper/right_2.png",
            "/assets/creatures/creeper/bottom_0.png",
            "/assets/creatures/creeper/bottom_1.png",
            "/assets/creatures/creeper/bottom_2.png",
            "/assets/creatures/creeper/top_0.png",
            "/assets/creatures/creeper/top_1.png",
            "/assets/creatures/creeper/top_2.png",
            "/assets/creatures/creeper/left_0.png",
            "/assets/creatures/creeper/left_1.png",
            "/assets/creatures/creeper/left_2.png",
            "/assets/interface/background.png",
            "/assets/interface/icon_lifes.png",
            "/assets/towers/orc/level 1/full.png",
            "/assets/towers/orc/level 1/icon.png",
            "/assets/towers/orc/shoot.png",
            "/assets/tiles/map.png",
            "/assets/interface/icon_score.png",
            "/assets/interface/slot.png",
        ];

        for asset in assets.iter() {
            self.texture_cache.load_texture(asset).await;
        }

        unsafe {
            log("All visual assets loaded.");
        };
        Ok(String::from("All visual assets loaded."))
    }
}

impl GameRenderer for BrowserRenderer {
    fn draw(&mut self, sprites: &Vec<Sprite>) -> Result<(), String> {
        self.canvas.set_fill_style(&"rgb(5,5,5)".into());
        self.canvas.fill_rect(0.0, 0.0, 900.0, 900.0);

        for sprite in sprites {
            match sprite.sprite_type {
                SpriteType::Image => {
                    // self.texture_cache.load_texture(&sprite.texture_path);
                    let x: f64 = sprite.position.x.into();
                    let y: f64 = sprite.position.y.into();

                    let texture = self
                        .texture_cache
                        .get_texture(&sprite.texture_path)
                        .texture
                        .as_ref();

                    match texture {
                        Some(_texture) => {
                            // _texture.set_width(400);

                            self.canvas
                                .draw_image_with_html_image_element_and_dw_and_dh(
                                    &_texture,
                                    x,
                                    y,
                                    sprite.width as f64,
                                    sprite.height as f64,
                                )
                                .unwrap();
                        }
                        None => {
                            let mut msg = "Can't render texture in renderer. Texture: ".to_owned();
                            msg.push_str(&sprite.texture_path);
                            unsafe {
                                error(&msg);
                            };
                            break;
                        }
                    }
                }
                SpriteType::Rect => {
                    let mut fill_style = "rgba(".to_owned();
                    fill_style.push_str(&sprite.color.r.to_string());
                    fill_style.push_str(",");
                    fill_style.push_str(&sprite.color.g.to_string());
                    fill_style.push_str(",");
                    fill_style.push_str(&sprite.color.b.to_string());
                    fill_style.push_str(",");
                    fill_style.push_str(&((sprite.color.a as f64) / 255.0).to_string());
                    fill_style.push_str(")");

                    self.canvas.set_fill_style(&fill_style.into());
                    self.canvas.fill_rect(
                        sprite.position.x as f64,
                        sprite.position.y as f64,
                        sprite.width as f64,
                        sprite.height as f64,
                    );
                    // self.canvas
                }
                SpriteType::Text => {
                    let r = "255";
                    let g = "255";
                    let b = "255";
                    let a = "255";
                    let mut fill_style = "rgba(".to_owned();
                    fill_style.push_str(r);
                    fill_style.push_str(",");
                    fill_style.push_str(g);
                    fill_style.push_str(",");
                    fill_style.push_str(b);
                    fill_style.push_str(",");
                    fill_style.push_str(a);
                    fill_style.push_str(")");
                    self.canvas.set_fill_style(&fill_style.into());
                    self.canvas.set_text_baseline("top");

                    let mut font_style = sprite.font_size.to_string();
                    font_style.push_str("px arial");
                    self.canvas.set_font(&font_style);

                    self.canvas.fill_text(
                        &sprite.text,
                        sprite.position.x as f64,
                        sprite.position.y as f64,
                    );
                }
            }
        }

        Ok(())
    }
}

// let cb = Closure::wrap(Box::new(move || {
//     alert("xyz");
//     // log("cb ran");
//     // run();
//     window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
//             .expect("failed requesting animation frame");
// }) as Box<dyn FnMut()>);

// Schedule the animation frame!
// let animation_id =
//     window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("failed requesting animation frame");

// log(&animation_id.to_string());

// // Again, return a handle to JS, so that the closure is not dropped
// // immediately and JS can decide whether to cancel the animation frame.
// AnimationFrameHandle {
//     animation_id,
//     _closure: cb,
// }
