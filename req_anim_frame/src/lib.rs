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
    // we update the window here just to reference the buffer
    // internally. Next calls to .update() will use the same buffer
    window
        .update_with_buffer(&buffer, WIDTH, HEIGHT)
        .unwrap();
    let mut pos_x : u32 = 0;
    let mut pos_y : u32 = 0;
    let mut step_x : i32 = 1;
    let mut step_y : i32 = 2;
    let cube_size = 50;
    // create the closure for updating and rendering the game.
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for val in buffer.iter_mut() {
            *val = 0; // clear screen
        }
        for i in 0..cube_size{
            for j in 0..cube_size{
                let pixel = i+pos_x+(j+pos_y)*WIDTH as u32;
                buffer[pixel as usize] = 0xFF42f5ad; //ABGR
            }
        }
        pos_x = (pos_x as i32 + step_x) as u32;
        pos_y = (pos_y as i32 + step_y) as u32;
        
        if pos_x <= 0 || pos_x + cube_size >= WIDTH as u32 {
            step_x = step_x * -1
        }
        if pos_y <= 0 || pos_y + cube_size >= HEIGHT as u32 {
            step_y = step_y * -1
        }

        // we could update_with_buffer here, but there is no need
        // as the buffer is referenced from inside the ImageData, and
        // we push that to the canvas
        window
            .update();
        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}


