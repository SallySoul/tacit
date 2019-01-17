#[macro_use]
extern crate assert;
extern crate cgmath;
extern crate geoprim;
extern crate itertools;
extern crate serde_json;

#[macro_use]
mod util;

pub mod function;
pub mod function_ir;
pub mod gen;
pub mod interval;
pub mod parser;
pub mod parser_error;
//pub mod mtree;
pub mod key;
pub mod mesh_tree;
