mod renderer;
use game::core::config::TILE_PIXEL_SIZE;
use game::core::Sprite;
use game::core::{GameRenderer, Point};
use game::game::Game;
use game::{core::config::MAP_TILES_AMOUNT_Y, game::MouseState};
use renderer::BrowserRenderer;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::EventListener;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
    fn cancelAnimationFrame(id: u32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = Rc::new(window.document().expect("no document"));

    let canvas = document.get_element_by_id("scene").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let context = Rc::new(context);

    let mut game = Game::new();
    game.start_round();

    let mut renderer = BrowserRenderer::new(context);

    renderer.load_assets().await.unwrap();

    let MOUSE_X: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let MOUSE_Y: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let MOUSE_BUTTONS_PRESSED: Rc<Cell<u16>> = Rc::new(Cell::new(0));

    setup_mouse_events_listeners(&MOUSE_X, &MOUSE_Y, &MOUSE_BUTTONS_PRESSED);

    let window = web_sys::window().expect("no global `window` exists");
    if let Some(perf) = window.performance() {
        let start_time = perf.now();

        // https://github.com/anlumo/webgl_rust_demo/blob/master/src/renderer.rs
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let outer_f = f.clone();

        let frames_rendered: Rc<Cell<i64>> = Rc::new(Cell::new(0));

        *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // log(&(perf.now() - start_time).to_string());
            let elapsed_time = perf.now() - start_time;

            // let mut coords = MOUSE_X.get().to_string();
            // coords.push_str(&MOUSE_Y.get().to_string());

            game.update(
                elapsed_time,
                MouseState::new(
                    MOUSE_BUTTONS_PRESSED.get() & 1 == 1,
                    Point::new(MOUSE_X.get(), MOUSE_Y.get()),
                ),
            );

            let mut sprites = game.get_sprites();

            let mut fps_msg = "FPS (calls): ".to_owned();
            let fps = frames_rendered
                .get()
                .checked_div(elapsed_time as i64 / 1000)
                .unwrap_or(0)
                .to_string();
            fps_msg.push_str(&fps);

            sprites.push(Sprite::create_text(
                &fps_msg,
                Point::new(10, TILE_PIXEL_SIZE * MAP_TILES_AMOUNT_Y as i32),
                16,
            ));

            renderer.draw(&sprites);

            frames_rendered.set(frames_rendered.get() + 1);

            // log("frames rendered: ");
            // log(&frames_rendered.get().to_string());

            // log(&fps.to_string());

            window
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("failed requesting animation frame");
        }) as Box<dyn FnMut()>));

        let window = web_sys::window().expect("no global `window` exists");

        window
            .request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }

    Ok(())
}

fn setup_mouse_events_listeners(
    mouse_x: &Rc<Cell<i32>>,
    mouse_y: &Rc<Cell<i32>>,
    mouse_buttons_pressed: &Rc<Cell<u16>>,
) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = Rc::new(window.document().expect("no document"));

    {
        let mouse_x_ref = mouse_x.clone();
        let mouse_y_ref = mouse_y.clone();
        let mouse_buttons_pressed_ref = mouse_buttons_pressed.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            mouse_x_ref.set(event.x());
            mouse_y_ref.set(event.y());
            mouse_buttons_pressed_ref.set(event.buttons());
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let mouse_x_ref = mouse_x.clone();
        let mouse_y_ref = mouse_y.clone();
        let mouse_buttons_pressed_ref = mouse_buttons_pressed.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            mouse_x_ref.set(event.x());
            mouse_y_ref.set(event.y());
            mouse_buttons_pressed_ref.set(event.buttons());
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}

// run_draw_loop();
// renderer.draw(&sprites);
// let monster = Rc::new(RefCell::new(Monster::new(
//     String::from("knight"),
//     spawn_point,
// )));

// let outer_monster = monster.clone();

// {
//     // let document = doc
//     // let context = context.clone();
//     // let pressed = pressed.clone();
//     // let monsterTwo = monster.clone().borrow_mut();
//     let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//         log("Keydown");

//         match event.key().as_ref() {
//             // "ArrowLeft" => { &game.entities.borrow_mut().get(0).unwrap().move_left(); },
//             // "ArrowRight" => outer_monster.borrow_mut().move_right(),
//             // "ArrowUp" => outer_monster.borrow_mut().move_up(),
//             // "ArrowDown" => outer_monster.borrow_mut().move_down(),
//             _ => {}
//         }
//     }) as Box<dyn FnMut(_)>);
//     document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
//     closure.forget();
// }

// pub fn eventlistener_keyboardevent_keydown(){
//     let window = web_sys::window().expect("global window does not exists");
//     let document = window.document().expect("expecting a document on window");

//     let on_keydown = EventListener::new(&document, "keydown", move |event| {

//     let keyboard_event = event.clone()
//                         .dyn_into::<web_sys::KeyboardEvent>()
//                         .unwrap();

//             let mut event_string = String::from("");
//             event_string.push_str(&event.type_());
//             event_string.push_str(&" : ");
//             event_string.push_str(&keyboard_event.key());
//             message.set_text_content(Some(&event_string));
//     });

//     on_keydown.forget();

// }
