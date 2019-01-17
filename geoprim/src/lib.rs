extern crate serde;
#[macro_use]
extern crate serde_derive;

/// A simple point 3D point
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LineSegment {
    pub p1: Point,
    pub p2: Point,
}

impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> LineSegment {
        LineSegment { p1, p2 }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Plot {
    pub lines: Vec<LineSegment>,
    pub points: Vec<Point>,
}

impl Plot {
    pub fn new() -> Plot {
        Plot {
            lines: Vec::new(),
            points: Vec::new(),
        }
    }

    pub fn add_line(&mut self, l: LineSegment) {
        self.lines.push(l);
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
}
