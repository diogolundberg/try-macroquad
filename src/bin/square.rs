#![windows_subsystem = "windows"]

use macroquad::prelude::*;

#[macroquad::main("A bloody square")]
async fn main() {
    loop {
        const TEXT: &str = "A bloody square";

        let size = screen_height() / 2.;

        let square = Rect {
            x: screen_width() / 2. - size / 2.,
            y: screen_height() / 2. - size / 2.,
            w: size,
            h: size,
        };

        draw_rectangle(square.x, square.y, square.w, square.h, RED);
        draw_text(TEXT, square.x, square.y, size/7., WHITE);
        next_frame().await;
    }
}
