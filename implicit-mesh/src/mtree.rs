use function::*;
use interval::Interval;
use interval::contains_zero;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;
use geoprim::*;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl BoundingBox {
    pub fn split(&self) -> Vec<BoundingBox> {
        let x_is = self.x.split();
        let y_is = self.y.split();
        let z_is = self.z.split();

        x_is.iter()
            .cartesian_product(y_is.iter())
            .cartesian_product(z_is.iter())
            .map(|((x, y), z)| {
                BoundingBox {
                    x: x.clone(),
                    y: y.clone(),
                    z: z.clone(),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct MNode {
    pub intervals: Vec<Interval>,
    pub bb: BoundingBox,
    pub children: Option<Vec<MNode>>,
}

impl MNode {
    pub fn split<F: Function>(
        &mut self,
        f: &Box<F>,
    ) {
        let children = self.bb
            .split()
            .iter()
            .map(|bb| {
                let mut bindings = HashMap::new();
                bindings.insert('x', bb.x);
                bindings.insert('y', bb.y);
                bindings.insert('z', bb.z);
                let intervals = f.evaluate_interval(&bindings);
                MNode {
                    bb: bb.clone(),
                    intervals: intervals,
                    children: None,
                }
            })
            .collect();

        self.children = Some(children);
    }

    fn contains_zero(&self) -> bool {
        contains_zero(&self.intervals)
    }

    pub fn find_roots<F: Function> (&mut self, f: &Box<F>, eps: f32) {
        let bb = self.bb.clone();
        if (bb.x.max - bb.x.min) < eps{
            return;
        }
        
        let recur = self.contains_zero();
        if recur {
            self.split(f);

            if let Some(children) = &mut self.children {
                for mut child in children {
                    child.find_roots(f, eps);
                }
            }
        }
    }

    pub fn add_to_plot(&self, min_plot: bool, plot: &mut Plot ) {
        let bb = self.bb;
        let is_min = self.contains_zero() && self.children.is_none();

        if (min_plot && is_min) || ! min_plot {
            // Build up the outline of a cube
            //
            // 1.) Make a point buffer with all the corners
            let mut points = Vec::new();
            for x in vec![bb.x.min, bb.x.max] {
                for y in vec![bb.y.min, bb.y.max] {
                    for z in vec![bb.z.min, bb.z.max] {
                        points.push(Point::new(x, y, z));
                    }
                }
            }

            // 2.) make a line buffer with appropriate endpoints
            let index_pairs = vec![
                (0, 1),
                (1, 3),
                (3, 2),
                (2, 0),
                (4, 5),
                (5, 7),
                (7, 6),
                (6, 4),
                (0, 4),
                (1, 5),
                (3, 7),
                (2, 6)
            ];

            for (p1, p2) in index_pairs {
                plot.add_line(LineSegment::new(points[p1], points[p2]));
            }
        }

        if  is_min {
            plot.add_point(Point::new(bb.x.middle(), bb.y.middle(), bb.z.middle()));
        }

        if let Some(ref children) = self.children {
            for c in children {
                c.add_to_plot(min_plot, plot);
            }
        }
    }
}
