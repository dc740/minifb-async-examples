use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut boxy = web_app::my_game::BouncyBox::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Bouncy Box demo",
        boxy.window_width,
        boxy.window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        boxy.game_step(&window);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(boxy.get_buffer_to_print(), WIDTH, HEIGHT)
            .unwrap();
    }
}
