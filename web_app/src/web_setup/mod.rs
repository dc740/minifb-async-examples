pub(crate) mod error;

use console_error_panic_hook;
use minifb::{Window, WindowOptions};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::my_game::BouncyBox;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut boxy = BouncyBox::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Bouncy Box demo",
        boxy.window_width,
        boxy.window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // A reference counted pointer to the closure that will update and render the game
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    // we update the window here just to reference the buffer
    // internally. Next calls to .update() will use the same buffer
    window
        .update_with_buffer(boxy.get_buffer_to_print(), WIDTH, HEIGHT)
        .unwrap();
    // create the closure for updating and rendering the game.
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        boxy.game_step(&window);

        // as the buffer is referenced from inside the ImageData, and
        // we push that to the canvas, so we could call update() and
        // avoid all this. I don't think it's possible to get artifacts
        // on the web side, but I definitely see them on the desktop app
        window.update_with_buffer(boxy.get_buffer_to_print(), WIDTH, HEIGHT);
        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}
