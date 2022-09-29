use crate::frame::{Drawable, Frame};

#[derive(Default)]
pub struct Score {
    count: u16,
}

impl Score {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn add_points(&mut self, amount: u16) {
        self.count += amount;
    }
}

impl Drawable for Score {
    fn draw(&self, frame:&mut Frame) {
        let formatted = format!("SCORE: {:0>4}", self.count);
        for (i, c) in formatted.chars().enumerate() {
            frame[i][0] = c;
        }
    }
}
