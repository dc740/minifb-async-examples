
/**
 * This is a bouncy box. No more, no less.
 * Bouncing through time in this ephimeral hologram of existence.
 * If someone finds this code in the future, this bouncy box
 * will keep bouncing decades after you are long gone and
 * forgotten. It's just a box. A bouncy box. And yet its
 * legacy is timeless. You are not. Somehow you think it's
 * OK to keep wasting your time reading this description. Curious
 * isn't it? You could be changing the world, but you are reading
 * a delusional description about a bouncy box demo.
 * How far are you willing to cotinue wasting your time on earth?
 * Probably longer than I could write. I should probably stop
 * procastrinating and get this demo done once and for all.
 * I was not that bored to keep writting anyway. What are
 * you doing with your life?
 */
pub struct BouncyBox {
    pub buffer: Vec<u32>,
    pub window_width:usize,
    pub window_height:usize,
    pos_x : u32,
    pos_y : u32,
    step_x : i32,
    step_y : i32,
    cube_size : u32
}

impl BouncyBox {
    pub fn new(window_width: usize, window_height:usize) -> BouncyBox{
        let buffer: Vec<u32> = vec![0; window_width * window_height];
        let pos_x : u32 = 0;
        let pos_y : u32 = 0;
        let step_x : i32 = 1;
        let step_y : i32 = 2;
        let cube_size = 50;
        BouncyBox {
            buffer,
            window_width,
            window_height,
            pos_x,
            pos_y,
            step_x,
            step_y,
            cube_size
        }
    }
    pub fn clear_screen(&mut self) {
        for val in self.buffer.iter_mut() {
            *val = 0; // clear screen
        }
    }
    pub fn game_step(&mut self) {
        for i in 0..self.cube_size{
            for j in 0..self.cube_size{
                let pixel = i+self.pos_x+(j+self.pos_y)*self.window_width as u32;
                self.buffer[pixel as usize] = 0xFF42F5AD; //ABGR
            }
        }
        self.pos_x = (self.pos_x as i32 + self.step_x) as u32;
        self.pos_y = (self.pos_y as i32 + self.step_y) as u32;
        
        if self.pos_x == 0 || self.pos_x + self.cube_size >= self.window_width as u32 {
            self.step_x *= -1
        }
        if self.pos_y == 0 || self.pos_y + self.cube_size >= self.window_height as u32 {
            self.step_y *= -1
        }
    }
}