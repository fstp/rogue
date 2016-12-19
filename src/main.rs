extern crate piston_window;
extern crate rand;

use piston_window::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const BLOCK_SIZE: f64 = 10.0;
const X_NUM_BLOCKS: u32 = SCREEN_WIDTH / BLOCK_SIZE as u32;
const Y_NUM_BLOCKS: u32 = SCREEN_HEIGHT / BLOCK_SIZE as u32;

struct Data {
    t: f32,
    x: u32,
    y: u32,
}

fn get_y(f: f32, a: f32, t: f32) -> u32 {
    let s = (f * t).sin() * a;
    let normalized = (s + 1.0) / 2.0;
    Y_NUM_BLOCKS - (Y_NUM_BLOCKS as f32 * normalized).floor() as u32
}

fn get_x(t: f32) -> u32 {
    (X_NUM_BLOCKS as f32 * t).floor() as u32
}

fn render<E>(e: &E, window: &mut PistonWindow, t: f32, f: f32, a: f32)
    where E: GenericEvent
{
    let d = Data {
        t: t,
        x: get_x(t),
        y: get_y(f, a, t),
    };

    window.draw_2d(e, |c, g| {
        clear([0.0; 4], g);
        for y in 0..Y_NUM_BLOCKS {
            for x in 0..X_NUM_BLOCKS {
                let pos = [BLOCK_SIZE * x as f64, BLOCK_SIZE * y as f64];
                render_square(&c, g, color(x, y, &d), pos)
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

fn color(x: u32, y: u32, d: &Data) -> [f32; 4] {
    let path = d.x == x && d.y == y;

    let r = if path {
        1.0 - d.t
    } else {
        (1.0 / X_NUM_BLOCKS as f32) * x as f32
    };

    let b = if path {
        1.0 * d.t
    } else {
        (1.0 / Y_NUM_BLOCKS as f32) * y as f32
    };

    [r, 0.0, b, 1.0]
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rogue", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .fullscreen(true)
        .vsync(true)
        .build()
        .unwrap();
    let mut t: f32 = 0.0;
    let mut dt: f32 = 0.0005;
    let mut f: f32 = 16.0;
    let mut a: f32 = 0.5;

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Q) => break,
                Button::Keyboard(Key::F) => {
                    f += 0.5;
                    t = 0.0
                }
                Button::Keyboard(Key::V) => {
                    f -= if f <= 0.0 { 0.0 } else { 0.5 };
                    t = 0.0
                }
                Button::Keyboard(Key::A) => {
                    a += 0.025;
                    t = 0.0
                }
                Button::Keyboard(Key::Z) => {
                    a -= if a <= 0.0 { 0.0 } else { 0.025 };
                    t = 0.0
                }
                Button::Keyboard(Key::S) => {
                    dt += 0.0005;
                    t = 0.0
                }
                Button::Keyboard(Key::X) => {
                    dt -= if dt <= 0.0 { 0.0 } else { 0.0005 };
                    t = 0.0
                }
                Button::Keyboard(Key::R) => {
                    t = 0.0;
                    dt = 0.0005;
                    f = 16.0;
                    a = 0.5;
                }
                _ => {}
            }
        }
        render(&e, &mut window, t, f, a);
        t += dt;
        t %= 1.0;
    }

}
