#![windows_subsystem = "windows"]

use macroquad::prelude::*;

use render_things::{files, Animation};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Character {
    pos: Point,
    vel: f32,
    size: Vec2,
    animation: Animation,
    source: Rect,
}

impl Character {
    fn render(&mut self) -> Rect {
        self.pos.x += self.vel;
        self.source.x = self.animation.next_frame();
        self.source
    }
}

#[macroquad::main("Running adventurer")]
async fn main() {
    let assets = files();
    let bytes = assets["adventurer.png"].clone();
    let texture: Texture2D = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));

    let mut adventurer = Character {
        pos: Point { x: 0., y: 0. },
        vel: screen_width() / 50.,
        size: vec2(screen_width() / 6., screen_height() / 6.),
        animation: Animation {
            frames: vec![50., 100., 150., 200., 250., 300.],
            current_frame: 0,
        },
        source: Rect {
            x: 0.,
            y: 45.,
            w: 50.,
            h: 28.,
        },
    };

    let mut last_render_time = get_time();

    loop {
        let time_passed = get_time();

        if time_passed - last_render_time > 1. / 8. {
            last_render_time = time_passed;

            if adventurer.pos.x < 0.
                || adventurer.pos.x
                    >= screen_width() - adventurer.source.w * adventurer.size.x / 60.
            {
                adventurer.vel = -adventurer.vel;
            }

            draw_texture_ex(
                texture,
                adventurer.pos.x,
                adventurer.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(adventurer.size),
                    source: Some(adventurer.render()),
                    flip_x: adventurer.vel < 0.,
                    ..Default::default()
                },
            );

            next_frame().await
        }
    }
}
