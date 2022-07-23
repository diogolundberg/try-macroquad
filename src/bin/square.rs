#![windows_subsystem = "windows"]

use macroquad::prelude::*;

#[macroquad::main("A bloody square")]
async fn main() {
    loop {
        const TEXT: &str = "A bloody square";

        let mut square = Rect {
            x: screen_width() / 2. - 100.,
            y: screen_height() / 2. - 100.,
            w: 200.,
            h: 200.,
        };

        draw_rectangle(square.x, square.y, square.w, square.h, RED);
        draw_text(TEXT, square.x, square.y, 30., WHITE);
        next_frame().await;
    }
}
