use color::Color;

const DEFAULT_WIDTH: f32 = 1.0;
const DEFAULT_COLOR: Color = color::BLACK;

pub struct LineSystemConstructor {
    width: f32,
    color: Color
}

impl LineSystemConstructor {
    pub fn new() -> LineSystemConstructor {
        LineSystemConstructor {
            width: DEFAULT_WIDTH,
            color: DEFAULT_COLOR
        }
    }

    pub fn finish(self) -> LineSystem {
        LineSystem {
            width: self.width,
            color: self.color,
        }
    }
}

pub struct LineSystem {
    width: f32,
    color: Color;
}

pub struct LineBuffers {
    vertices_float_buffer: 
}
