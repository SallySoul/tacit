use super::color;
use super::color::Color;
use smallvec::SmallVec;
use std::mem;
use web_sys::WebGlRenderingContext;

const DEFAULT_WIDTH: f32 = 1.0;
const DEFAULT_COLOR: Color = color::BLACK;
const MAX_POINTS_PER_BUFFER: usize = (u16::max_value() / 3) as usize;

pub struct PointSystemConstructor {
    width: f32,
    color: Color,
    active_float_buffer: Vec<f32>,
    active_point_count: usize,
    filled_float_buffers: SmallVec<[Vec<f32>; 1]>,
}

impl PointSystemConstructor {
    pub fn new() -> PointSystemConstructor {
        PointSystemConstructor {
            width: DEFAULT_WIDTH,
            color: DEFAULT_COLOR,
            active_float_buffer: Vec::new(),
            active_point_count: 0,
            filled_float_buffers: SmallVec::new(),
        }
    }

    pub fn set_width(&mut self, width: f32) -> &mut PointSystemConstructor {
        self.width = width;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut PointSystemConstructor {
        self.color = color;
        self
    }

    pub fn add_point(&mut self, position: &[f32; 3]) -> &mut PointSystemConstructor {
        if self.active_point_count < MAX_POINTS_PER_BUFFER {
            // Swap out the active (and now full) buffer for an empty one
            let mut full_buffer = Vec::new();
            mem::swap(&mut full_buffer, &mut self.active_float_buffer);

            // Keep track of full buffer, cleanup, and add point
            self.filled_float_buffers.push(full_buffer);
            self.active_point_count = 0;
            self.add_point(position)
        } else {
            self.active_float_buffer.extend(position);
            self.active_point_count += 1;
            self
        }
    }

    pub fn finish(self, gl_context: &WebGlRenderingContext) -> PointSystem {
        PointSystem {
            width: self.width,
            color: self.color,
        }
    }
}

pub struct PointBuffer {}

pub struct PointSystem {
    width: f32,
    color: Color,
}
