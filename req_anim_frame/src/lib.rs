extern crate minifb;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;
use std::panic;
use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];


    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // A reference counted pointer to the closure that will update and render the game
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // create the closure for updating and rendering the game.
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for val in buffer.iter_mut() {
            if *val == 0 {*val = 0xFFFFFFFF;}
            else {*val = 0;}
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}


