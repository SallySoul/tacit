use crate::function::Function;
use crate::interval::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Exp(Box<Node>, Box<Node>),
    Variable(char),
    Constant(f32),
}

impl Node {
    pub fn evaluate(&self, bindings: &HashMap<char, f32>) -> f32 {
        match *self {
            Node::Add(ref n1, ref n2) => n1.evaluate(&bindings) + n2.evaluate(&bindings),
            Node::Sub(ref n1, ref n2) => n1.evaluate(&bindings) - n2.evaluate(&bindings),
            Node::Mul(ref n1, ref n2) => n1.evaluate(&bindings) * n2.evaluate(&bindings),
            Node::Div(ref n1, ref n2) => n1.evaluate(&bindings) / n2.evaluate(&bindings),
            Node::Exp(ref n1, ref n2) => n1.evaluate(&bindings).powf(n2.evaluate(&bindings)),
            Node::Constant(c) => c,
            Node::Variable(v) => bindings.get(&v).unwrap().clone(),
        }
    }

    pub fn evaluate_intervals(&self, bindings: &HashMap<char, Interval>) -> Vec<Interval> {
        match *self {
            Node::Add(ref n1, ref n2) => {
                permute_intervals(&n1, &n2, &bindings, |(interval1, interval2)| {
                    interval1.add(interval2)
                })
            }
            Node::Sub(ref n1, ref n2) => {
                permute_intervals(&n1, &n2, &bindings, |(interval1, interval2)| {
                    interval1.sub(interval2)
                })
            }
            Node::Mul(ref n1, ref n2) => {
                permute_intervals(&n1, &n2, &bindings, |(interval1, interval2)| {
                    interval1.mul(interval2)
                })
            }
            Node::Exp(ref n1, ref n2) => {
                permute_intervals(&n1, &n2, &bindings, |(interval1, interval2)| {
                    interval1.exp(interval2)
                })
            }
            Node::Div(ref n1, ref n2) => {
                permute_intervals(&n1, n2, &bindings, |(interval1, interval2)| {
                    interval1.div(interval2)
                })
            }
            Node::Constant(c) => vec![Interval { min: c, max: c }],
            Node::Variable(v) => vec![bindings.get(&v).unwrap().clone()],
        }
    }
}

impl Function for Node {
    fn evaluate(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut bindings = HashMap::new();
        bindings.insert('x', x);
        bindings.insert('y', y);
        bindings.insert('z', z);

        self.evaluate(&bindings)
    }

    fn evaluate_interval(&self, bindings: &HashMap<char, Interval>) -> Vec<Interval> {
        self.evaluate_intervals(&bindings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_expression;
    use std::collections::HashMap;

    #[test]
    fn test_function_evaluate() {
        let mut input: Vec<char>;
        let mut root;
        let mut bindings = HashMap::new();
        bindings.insert('x', 1.13);
        bindings.insert('y', 4.232);
        bindings.insert('z', 2.0939);

        input = "x".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 1.13);

        input = "x + y ^ z".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 21.6380);

        input = "x + y - z".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 3.2681);

        input = "x + y - z / x - y + z".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 1.3709);

        input = "x + y - (z / x) - y + z".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 1.3709);

        input = "3.2 ^ (0.01 / 8) + (4.0 * 3 + 2 - 3^7 - (4)) / z ^ 2"
            .chars()
            .collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), -495.5297);

        input = "0".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        assert_similiar!(root.evaluate(&bindings), 0.0);
    }

    #[test]
    fn test_function_inteval() {
        let mut input: Vec<char>;
        let mut root;
        let mut result;
        let mut bindings = HashMap::new();
        bindings.insert(
            'x',
            Interval {
                min: 0.01,
                max: 3.1,
            },
        );
        bindings.insert(
            'y',
            Interval {
                min: -5.0,
                max: 5.0,
            },
        );
        bindings.insert(
            'z',
            Interval {
                min: -3.0,
                max: -1.0,
            },
        );

        input = "x".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        result = root.evaluate_interval(&bindings);
        assert_eq!(result.len(), 1);
        assert_similiar!(result[0].min, 0.01);
        assert_similiar!(result[0].max, 3.1);

        input = "x+y".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        result = root.evaluate_interval(&bindings);
        assert_eq!(result.len(), 1);
        assert_similiar!(result[0].min, -4.99);
        assert_similiar!(result[0].max, 8.1);
        assert!(result[0].contains_zero());
        assert!(contains_zero(&vec![result[0]]));

        input = "x*y".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        result = root.evaluate_interval(&bindings);
        assert_eq!(result.len(), 1);
        assert_similiar!(result[0].min, -15.5);
        assert_similiar!(result[0].max, 15.5);

        // TDOD add more tests once behaivor settles
    }

    #[test]
    fn test_function_inteval_2() {
        let mut input: Vec<char>;
        let mut root;
        let mut result;
        let mut bindings = HashMap::new();
        bindings.insert(
            'x',
            Interval {
                min: -1.0,
                max: 1.0,
            },
        );
        bindings.insert(
            'y',
            Interval {
                min: -1.0,
                max: 1.0,
            },
        );
        bindings.insert(
            'z',
            Interval {
                min: -1.0,
                max: 1.0,
            },
        );

        input = "x-y".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        result = root.evaluate_interval(&bindings);
        assert!(result[0].contains_zero());

        input = "0".chars().collect();
        root = parse_expression(&input, 0).unwrap();
        result = root.evaluate_interval(&bindings);
        assert!(result[0].contains_zero());
    }

}
