#![allow(unused_variables)]
#![allow(dead_code)]

// The assert crate is a dev dependency
// However I need it for unit testing, so I think I need it here?
extern crate assert;
extern crate cgmath;
#[macro_use]
extern crate getset;

mod camera;
mod perspective;

pub use crate::camera::ButtonState;
pub use crate::camera::Camera;
pub use crate::camera::MouseButton;
pub use crate::perspective::fov_perspective_transform;
pub use crate::perspective::perspective_transform;
