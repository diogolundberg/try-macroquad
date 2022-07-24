#![windows_subsystem = "windows"]

use macroquad::prelude::*;

use render_things::{files, Animation};

#[macroquad::main("Idle adventurer")]
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
