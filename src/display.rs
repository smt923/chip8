use sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use chip8::{SCREEN_H, SCREEN_W, Chip8};

pub const SCALE: i32 = 6;

pub struct Display {
    pub canvas: Canvas<Window>,
    pub ctx: sdl2::Sdl,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP8", 64*SCALE as u32, 32*SCALE as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Display {
            canvas: canvas,
            ctx: sdl_context,
        }
    }

    pub fn draw(&mut self, gfx: &[u8]) {
        let mut rects = vec![];
        for x in 0..SCREEN_W {
            for y in 0..SCREEN_H {
                if gfx[x + y * SCREEN_W] == 1 {
                    rects.push(Rect::new((x as i32) * SCALE, (y as i32) * SCALE, SCALE as u32, SCALE as u32));
                }
            }
        }
        match self.canvas.fill_rects(&rects) {
            Ok(()) => self.canvas.present(),
            Err(why) => println!("{}", why)
        };
    }

    /// Shortcut to clearing our screen with the correct colors
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(20, 20, 20));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(220, 220, 220));
    }
}