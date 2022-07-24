#![windows_subsystem = "windows"]

// use std::env;

use std::collections::HashMap;

use include_dir::{include_dir, Dir};
use macroquad::prelude::*;

struct Animation {
    frames: Vec<f32>,
    current_frame: usize,
}

impl Animation {
    fn next_frame(&mut self) -> f32 {
        self.current_frame += 1;
        if self.current_frame >= self.frames.len() {
            self.current_frame = 0;
        }
        return self.frames[self.current_frame];
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "A adventurer".to_owned(),
        window_width: 250,
        window_height: 200,
        ..Default::default()
    }
}

static ASSETS: Dir = include_dir!("assets");

fn files() -> HashMap<String, &'static [u8]> {
    ASSETS
        .files()
        .map(|f| {
            (
                f.path().file_name().unwrap().to_str().unwrap().to_owned(),
                f.contents(),
            )
        })
        .collect()
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = files();
    let bytes = assets["adventurer.png"].clone();
    let texture: Texture2D = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));

    let mut animation = Animation {
        frames: vec![0., 50., 100., 150.],
        current_frame: 0,
    };
    let mut last_render_time = get_time();

    loop {
        let time_passed = get_time();

        if time_passed - last_render_time > 1. / 5. {
            last_render_time = time_passed;
            let frame: f32 = Animation::next_frame(&mut animation);

            let adventurer = Rect {
                x: frame,
                y: 0.,
                w: 50.,
                h: 40.,
            };

            draw_texture_ex(
                texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(250., 200.)),
                    source: Some(adventurer),
                    ..Default::default()
                },
            );

            next_frame().await
        }
    }
}
