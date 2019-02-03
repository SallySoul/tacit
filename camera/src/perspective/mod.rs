use cgmath::Matrix4;

/// Create a perspective transform that takes eye space coordinates into clip space
/// using all the specification needed to defined a viewing frustrum
pub fn perspective_transform(
    near: f32,
    far: f32,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
) -> Matrix4<f32> {
    let r1c1 = (2.0 * near) / (right - left);
    let r1c3 = (right + left) / (right - left);
    let r2c2 = (2.0 * near) / (top - bottom);
    let r2c3 = (top + bottom) / (top - bottom);
    let r3c3 = -(far + near) / (far - near);
    let r3c4 = -(2.0 * far * near) / (far - near);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new(
        r1c1, 0.0,  0.0,  0.0,
        0.0,  r2c2, 0.0,  0.0,
        r1c3, r2c3, r3c3, -1.0,
        0.0,  0.0,  r3c4, 0.0
    )
}

/// Create a perspective transform that takes eye space coordinates into clip space
/// using field of view and aspect ratio to define the viewing frustrum. We assume
/// that the center of the near plane is also the center of the screen.
pub fn fov_perspective_transform(
    field_of_view: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> Matrix4<f32> {
    let top = near * (field_of_view / 2.0).tan();
    let bottom = -top;
    let right = top * aspect_ratio;
    let left = -right;
    perspective_transform(near, far, left, right, bottom, top)
}

/// The near plane distance is dependent on the field of view. It is useful to have this
/// calculation be seperate
pub fn fov_near_distance(field_of_view: f32) -> f32 {
    1.0 / (field_of_view / 2.0).tan()
}

/*
// Create an orthographic transform
/// That takes eye space coordiantes into clip space
pub fn orthographic_transform(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
) -> Matrix4<f32> {
    let r1c1 = 0.0;

    let r2c2 = 0.0;

    let r3c3 = 0.0;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new(
        r1c1, 0.0,  0.0,  0.0,
        0.0,  r2c2, 0.0,  0.0,
        r1c3, r2c3, r3c3, -1.0,
        0.0,  0.0,  r3c4, 0.0
    )
}
*/
