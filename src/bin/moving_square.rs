#![windows_subsystem = "windows"]

use std::f32::consts;

use macroquad::prelude::*;

#[macroquad::main("A moving square")]
async fn main() {
    const TEXT: &str = "A moving square";
    const SPEED: f32 = 7.;

    let mut square = Rect {
        x: screen_width() / 2. - 100.,
        y: screen_height() / 2. - 100.,
        w: 200.,
        h: 200.,
    };

    let mut angle: f32 = 2.;
    let mut last_render_time = get_time();

    loop {
        let time_passed = get_time();

        if time_passed - last_render_time > 1. / 60. {
            last_render_time = time_passed;

            if square.x < 0. || square.x >= screen_width() - square.w {
                angle = (angle + consts::PI) * -1.;
            }

            if square.y < 0. || square.y > screen_height() - square.h {
                angle = -angle;
            }

            square.move_to(vec2(
                square.x + angle.cos() * SPEED,
                square.y + angle.sin() * SPEED,
            ));

            draw_rectangle(square.x, square.y, square.w, square.h, RED);
            draw_text(TEXT, square.x, square.y, 30., WHITE);

            next_frame().await;
        }
    }
}
