#![windows_subsystem = "windows"]

use std::f32::consts;

use macroquad::prelude::*;

#[macroquad::main("A moving square")]
async fn main() {
    const TEXT: &str = "A moving square";

    let size = screen_height() / 6.;

    let mut square = Rect {
        x: screen_width() / 2. - size / 2.,
        y: screen_height() / 2. - size / 2.,
        w: size,
        h: size,
    };

    let mut angle: f32 = 2.;
    let mut last_render_time = get_time();

    loop {
        let time_passed = get_time();

        if time_passed - last_render_time > 1. / 200. {
            last_render_time = time_passed;

            if square.x < 0. || square.x >= screen_width() - square.w {
                angle = (angle + consts::PI) * -1.;
            }

            if square.y < 0. || square.y > screen_height() - square.h {
                angle = -angle;
            }

            square.move_to(vec2(
                square.x + angle.cos() * size / 20.,
                square.y + angle.sin() * size / 20.,
            ));

            draw_rectangle(square.x, square.y, square.w, square.h, RED);
            draw_text(TEXT, square.x, square.y, size / 6., WHITE);

            next_frame().await;
        }
    }
}
