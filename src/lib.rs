use std::collections::HashMap;

use include_dir::{include_dir, Dir};

pub static ASSETS: Dir = include_dir!("assets");

pub fn files() -> HashMap<String, &'static [u8]> {
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

pub struct Animation {
    pub frames: Vec<f32>,
    pub current_frame: usize,
}

impl Animation {
    pub fn next_frame(&mut self) -> f32 {
        self.current_frame += 1;
        if self.current_frame >= self.frames.len() {
            self.current_frame = 0;
        }
        return self.frames[self.current_frame];
    }
}
