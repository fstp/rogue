extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use rand::ThreadRng;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const BLOCK_SIZE: f64 = 10.0;
const X_NUM_BLOCKS: u32 = SCREEN_WIDTH / BLOCK_SIZE as u32;
const Y_NUM_BLOCKS: u32 = SCREEN_HEIGHT / BLOCK_SIZE as u32;

fn render<E>(e: &E, window: &mut PistonWindow)
    where E: GenericEvent
{
    let mut rng = rand::thread_rng();

    window.draw_2d(e, |c, g| {
        clear([0.0; 4], g);
        for y in 0..Y_NUM_BLOCKS {
            for x in 0..X_NUM_BLOCKS {
                let pos = [BLOCK_SIZE * x as f64, BLOCK_SIZE * y as f64];
                render_square(&c, g, color(x, y, &mut rng), pos)
            }
        }
    });
}

fn render_square<G: Graphics>(c: &Context, g: &mut G, color: [f32; 4], pos: [f64; 2]) {
    rectangle(color,
              [pos[0], pos[1], BLOCK_SIZE, BLOCK_SIZE],
              c.transform,
              g);
}

fn color(x: u32, y: u32, rng: &mut ThreadRng) -> [f32; 4] {
    let r = (1.0 / X_NUM_BLOCKS as f32) * x as f32;
    let b = (1.0 / Y_NUM_BLOCKS as f32) * y as f32;
    let g: f32 = rng.gen();
    [r, g, b, 1.0]
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rogue", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        // .fullscreen(true)
        .vsync(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Q) => break,
                _ => {}
            }
        }
        render(&e, &mut window);
    }

}
