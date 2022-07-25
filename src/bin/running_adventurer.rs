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

    let ratio = screen_height() / 200.;

    let mut adventurer = Character {
        pos: Point { x: 0., y: 0. },
        vel: 7. * ratio,
        animation: Animation {
            frames: vec![61., 111., 161., 211., 261., 311.],
            current_frame: 0,
        },
        source: Rect {
            x: 0.,
            y: 45.,
            w: 28.,
            h: 28.,
        },
    };

    let size = vec2(adventurer.source.w * ratio, adventurer.source.h * ratio);

    print!("{:?}", size);

    let mut last_render_time = get_time();

    loop {
        let time_passed = get_time();

        if time_passed - last_render_time > 1. / 8. {
            last_render_time = time_passed;

            if adventurer.pos.x < 0.
                || adventurer.pos.x
                    >= screen_width() - size.x
            {
                adventurer.vel = -adventurer.vel;
            }

            draw_texture_ex(
                texture,
                adventurer.pos.x,
                adventurer.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(size),
                    source: Some(adventurer.render()),
                    flip_x: adventurer.vel < 0.,
                    ..Default::default()
                },
            );

            next_frame().await
        }
    }
}
