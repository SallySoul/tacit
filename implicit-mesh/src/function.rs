use crate::interval::Interval;
use std::collections::HashMap;

pub trait Function: Sized {
    fn evaluate(&self, x: f32, y: f32, z: f32) -> f32;

    fn evaluate_interval(&self, bindings: &HashMap<char, Interval>) -> Vec<Interval>;
}

#[derive(Copy, Clone)]
pub struct ConstFunction {
    pub c: f32,
}

impl Function for ConstFunction {
    fn evaluate(&self, x: f32, y: f32, z: f32) -> f32 {
        self.c
    }

    fn evaluate_interval(&self, bindings: &HashMap<char, Interval>) -> Vec<Interval> {
        vec![Interval {
            min: self.c,
            max: self.c,
        }]
    }
}
