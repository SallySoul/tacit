use crate::function::Function;
use itertools::Itertools;
use std::collections::HashMap;
use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn add(&self, other: &Interval) -> Vec<Interval> {
        vec![Interval {
            min: self.min + other.min,
            max: self.max + other.max,
        }]
    }

    pub fn sub(&self, other: &Interval) -> Vec<Interval> {
        vec![Interval {
            min: self.min - other.max,
            max: self.max - other.min,
        }]
    }

    pub fn mul(&self, other: &Interval) -> Vec<Interval> {
        let minmax = [self.min, self.max]
            .iter()
            .cartesian_product(&[other.min, other.max])
            .map(|(min, max)| min * max)
            .minmax()
            .into_option()
            .unwrap();

        vec![Interval {
            min: minmax.0,
            max: minmax.1,
        }]
    }

    pub fn div(&self, other: &Interval) -> Vec<Interval> {
        let inverse = match (other.min, other.max) {
            (_, _) if !other.contains_zero() => Interval {
                min: 1.0 / other.min,
                max: 1.0 / other.max,
            },
            (min, max) if max == 0.0 => Interval {
                min: -f32::INFINITY,
                max: 1.0 / min,
            },
            (min, max) if min == 0.0 => Interval {
                min: 1.0 / max,
                max: f32::INFINITY,
            },
            (min, max) => Interval {
                min: -f32::INFINITY,
                max: f32::INFINITY,
            },
        };

        self.mul(&inverse)
    }

    pub fn exp(&self, power: &Interval) -> Vec<Interval> {
        let minmax = [self.min, self.max]
            .iter()
            .cartesian_product(&[power.min, power.max])
            .map(|(base, power)| base.powf(*power))
            .minmax()
            .into_option()
            .unwrap();

        vec![Interval {
            min: if self.contains_zero() { 0.0 } else { minmax.0 },
            max: minmax.1,
        }]
    }

    pub fn middle(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

    pub fn split(&self) -> [Interval; 2] {
        let middle = self.middle();
        [
            Interval {
                min: self.min,
                max: middle,
            },
            Interval {
                min: middle,
                max: self.max,
            },
        ]
    }

    pub fn contains_zero(&self) -> bool {
        self.min <= 0.0 && self.max >= 0.0
    }

    pub fn clamp_value(&self, v: f32) -> f32 {
        if v > self.max {
            self.max
        } else if v < self.min {
            self.min
        } else {
            v
        }
    }
}

pub fn permute_intervals<A, F>(
    node1: &Box<A>,
    node2: &Box<A>,
    bindings: &HashMap<char, Interval>,
    op: F,
) -> Vec<Interval>
where
    F: FnMut((&Interval, &Interval)) -> Vec<Interval>,
    A: Function,
{
    let n1_i = node1.evaluate_interval(&bindings);
    let n2_i = node2.evaluate_interval(&bindings);

    n1_i.iter().cartesian_product(&n2_i).map(op).concat()
}

pub fn contains_zero(intervals: &[Interval]) -> bool {
    for interval in intervals {
        if interval.contains_zero() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert::*;

    #[test]
    fn test_add() {
        let a = Interval::new(1.0, 2.0);
        let b = Interval::new(2.0, 3.0);
        let r = a.add(&b);
        close(r[0].min, 3.0, f32::EPSILON);
        close(r[0].max, 5.0, f32::EPSILON);

        let a = Interval::new(-1.0, 1.0);
        let b = Interval::new(-1.0, 1.0);
        let r = a.add(&b);
        close(r[0].min, -2.0, f32::EPSILON);
        close(r[0].max, 2.0, f32::EPSILON);

        let a = Interval::new(-1.0, 4.0);
        let b = Interval::new(2.0, 3.0);
        let r = a.add(&b);
        close(r[0].min, 1.0, f32::EPSILON);
        close(r[0].max, 7.0, f32::EPSILON);

        let a = Interval::new(-1.0, -0.5);
        let b = Interval::new(-2.0, -0.5);
        let r = a.add(&b);
        close(r[0].min, -3.0, f32::EPSILON);
        close(r[0].max, -1.0, f32::EPSILON);
    }

    #[test]
    fn test_sub() {
        let a = Interval::new(1.0, 2.0);
        let b = Interval::new(2.0, 3.0);
        let r = a.sub(&b);
        close(r[0].min, -2.0, f32::EPSILON);
        close(r[0].max, 0.0, f32::EPSILON);

        let a = Interval::new(-1.0, 1.0);
        let b = Interval::new(-1.0, 1.0);
        let r = a.sub(&b);
        close(r[0].min, -2.0, f32::EPSILON);
        close(r[0].max, 2.0, f32::EPSILON);

        let a = Interval::new(-1.0, 1.0);
        let b = Interval::new(-1.0, 1.0);
        let r = a.sub(&b);
        close(r[0].min, -2.0, f32::EPSILON);
        close(r[0].max, 2.0, f32::EPSILON);
    }
}
