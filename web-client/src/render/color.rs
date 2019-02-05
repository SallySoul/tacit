use std::ops::{Deref, DerefMut};

pub const RED: Color = Color {
    rgba: [1.0, 0.0, 0.0, 1.0],
};
pub const BLUE: Color = Color {
    rgba: [0.0, 0.0, 1.0, 1.0],
};
pub const GREEN: Color = Color {
    rgba: [0.0, 1.0, 0.0, 1.0],
};
pub const WHITE: Color = Color {
    rgba: [1.0, 1.0, 1.0, 1.0],
};
pub const GREY: Color = Color {
    rgba: [0.625, 0.625, 0.625, 1.0],
};
pub const BLACK: Color = Color {
    rgba: [0.0, 0.0, 0.0, 1.0],
};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub rgba: [f32; 4],
}

impl Deref for Color {
    type Target = [f32];

    fn deref(&self) -> &[f32] {
        &self.rgba
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut [f32] {
        &mut self.rgba
    }
}

impl Color {
    pub fn from_floats(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { rgba: [r, g, b, a] }
    }
    /*
        pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Color {
            Color {
                rgba: [
                    r as f32 / 256.0,
                    g as f32 / 256.0,
                    b as f32 / 256.0,
                    a as f32 / 256.0,
                ],
            }
        }
    */
}
