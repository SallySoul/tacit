use crate::perspective;
use cgmath::prelude::*;
use cgmath::{Basis3, Matrix3, Matrix4, Quaternion, Rad, Vector2, Vector3};
use std::f32;

/// The camera is a state machine, what each input does depends on the state that its in.
/// The possible states are this enum.
#[derive(PartialEq, Eq)]
enum CamState {
    /// This mode allows the user to move the camera target
    Pan,

    /// This mode allows the user to adjust the camera rotation
    Tumble,

    /// This state is used for have the camera animate to a destination state
    Transition,

    /// This state continues rotation from tumble mode when user lets go
    IdleOrbit,

    /// The camera is neither animating nor using mouse movement
    Idle,
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum ButtonState {
    Pressed,
    Released,
}

/// The camera struct maintains all the state of the camera. In order to maintain correct the
/// correct aspect ratio and timing for orbital mechanics, it needs to be updated every frame.
#[derive(Getters, Setters)]
pub struct Camera {
    // TODO wrap up camera state stuff into a camera state type struct
    state: CamState,
    window_width: f32,
    window_height: f32,
    aspect_ratio: f32,

    prev_mouse_coords: Vector2<f32>,

    default_target: Vector3<f32>,
    default_distance: f32,
    default_rotation: Quaternion<f32>,

    default_transition_duration: f32,

    // These are for maintaining the starting state when switching to
    // Tumble, Pan, and Transition
    original_rotation: Quaternion<f32>,
    original_distance: f32,
    original_sphere_point: Vector3<f32>,

    // These are for maintaining state with the pan calculations
    original_target: Vector3<f32>,
    original_pan_point: Vector3<f32>,

    // Orbit
    orbit_enabled: bool,
    orbit_velocity: f32,
    last_rotation: Quaternion<f32>,
    tumble_duration: f32,

    // Transition
    transition_end_rotation: Quaternion<f32>,
    transition_end_target: Vector3<f32>,
    transition_end_distance: f32,

    // milliseconds
    transition_duration: f32,
    transition_completed: f32,

    /// How far the camera is from the target in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    distance: f32,

    /// The distance from the camera to the near plane of the viewing frustrum
    #[get = "pub"]
    #[set = "pub"]
    near: f32,

    /// The distance from the camera to the far plane of the viewing frustrum
    #[get = "pub"]
    #[set = "pub"]
    far: f32,

    /// The field of view to use when making the perspective transform
    #[get = "pub"]
    #[set = "pub"]
    field_of_view: f32,

    /// How the camera is oriented relative to the target in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    rotation: Quaternion<f32>,

    /// The factor applied to the number of pixels from each scroll event,
    /// I have default set of (1 / 200)
    #[get = "pub"]
    #[set = "pub"]
    scroll_modifier: f32,

    /// The target is where the camera points in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    target: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            state: CamState::Idle,
            target: Vector3::new(0.0, 0.0, 0.0),
            distance: 50.0,
            prev_mouse_coords: Vector2::zero(),

            // Default state for camera transition
            default_target: Vector3::zero(),
            default_distance: 50.0,
            default_rotation: Quaternion::from(Basis3::from_angle_y(Rad(-0.5 * f32::consts::PI))),
            default_transition_duration: 450.0,

            // The state calculations can the identities, doesn't matter
            original_rotation: Quaternion::one(),
            original_target: Vector3::zero(),
            original_distance: 1.0,

            // These are the
            original_sphere_point: Vector3::zero(),
            original_pan_point: Vector3::zero(),

            // OrbitDelta
            orbit_velocity: 0.0,
            orbit_enabled: false,
            last_rotation: Quaternion::one(),
            tumble_duration: 1.0,

            transition_end_rotation: Quaternion::one(),
            transition_end_target: Vector3::zero(),
            transition_end_distance: 1.0,
            transition_duration: Default::default(),
            transition_completed: Default::default(),

            rotation: Quaternion::from(Basis3::from_angle_y(Rad(-0.5 * f32::consts::PI))),
            window_width: 1.0,
            window_height: 1.0,
            aspect_ratio: 1.0,
            field_of_view: f32::consts::PI / 2.0,
            near: 0.01,
            far: 1000.0,
            scroll_modifier: 1.0 / 200.0,
        }
    }

    /// The update function should be called once per frame in order to maintain the aspect ratio
    /// and the timing for orbital mechanics. Ideally it should be called at the begining of your
    /// "simulation loop", right after you have calculated your frame time.
    /// TODO: Duration should be a f32 millis too
    pub fn update(&mut self, elapsed_millis: f32, window_width: f32, window_height: f32) {
        self.window_width = window_width;
        self.window_height = window_height;
        self.aspect_ratio = window_width / window_height;

        match self.state {
            CamState::Transition => {
                self.transition_completed += elapsed_millis;

                if self.transition_completed >= self.transition_duration {
                    self.state = CamState::Idle;
                    self.rotation = self.transition_end_rotation;
                    self.target = self.transition_end_target;
                    self.distance = self.transition_end_distance;
                } else {
                    let t = self.transition_completed / self.transition_duration;

                    self.target = (1.0 - t) * self.original_target + t * self.transition_end_target;
                    self.distance =
                        (1.0 - t) * self.original_distance + t * self.transition_end_distance;
                    self.rotation = self
                        .original_rotation
                        .slerp(self.transition_end_rotation, t);
                }
            }
            CamState::IdleOrbit => {
                let mut current_angle = self.rotation.s.acos() * 2.0;
                current_angle += (elapsed_millis / 1000.0) * self.orbit_velocity;
                self.rotation.s = (current_angle / 2.0).cos();
            }
            CamState::Tumble => {
                self.tumble_duration += elapsed_millis;
            }
            _ => (),
        }
    }

    /// Use this to setup a camera transition
    pub fn start_transition(
        &mut self,
        end_target: Vector3<f32>,
        end_rotation: Quaternion<f32>,
        end_distance: f32,
        transition_duration: f32,
    ) {
        self.state = CamState::Transition;

        self.original_target = self.target;
        self.original_rotation = self.rotation;
        self.original_distance = self.distance;

        self.transition_end_target = end_target;
        self.transition_end_rotation = end_rotation;
        self.transition_end_distance = end_distance;
        self.transition_duration = transition_duration;
        self.transition_completed = 0.0;
    }

    // TODO, transitions should be moved to another module I think.
    // could have state stack, with default at bottom,
    // could also have snap to axis and thigs like that
    pub fn transition_to_default(&mut self) {
        let rotation = self.default_rotation.clone();
        let target = self.default_target.clone();
        let distance = self.default_distance.clone();
        let duration = self.default_transition_duration;
        self.start_transition(target, rotation, distance, duration);
    }

    pub fn set_current_as_default(&mut self) {
        self.default_rotation = self.rotation;
        self.default_distance = self.distance;
        self.default_target = self.target;
    }

    /// Get the position of the camera in world coordinates
    pub fn get_position(&self) -> Vector3<f32> {
        self.target
            + self
                .rotation
                .rotate_vector(Vector3::unit_z() * self.distance)
    }

    /// Get the rotation of the camera
    pub fn get_rotation(&self) -> Basis3<f32> {
        Basis3::from(self.rotation)
    }

    /// Get the world coordinates to clipspace coordinates transform
    /// If you are unsure, this is probably the transform you want from the camera.
    pub fn get_clipspace_transform(&self) -> Matrix4<f32> {
        // We need to move to transform the world so that the origin is the cam's pos
        let inverse_pos = -self.get_position();
        let pos_transform = Matrix4::from_translation(inverse_pos);

        let rotation_transform = Matrix3::from(self.get_rotation().invert());

        let perspective_transform = perspective::fov_perspective_transform(
            self.field_of_view,
            self.aspect_ratio,
            self.near,
            self.far,
        );

        // We need to an inverted order of operations becuase the matrix is inverted(?)
        perspective_transform * Matrix4::from(rotation_transform) * pos_transform
    }

    // When dealing with mouse input we need to translate the pixel location into
    // screenspace. Screenspace is a rectangle, and it must circumscribe the unit circle
    // When the screen is square, screen space is [-1, 1]^2
    fn mouse_to_screen(&self, mouse_coords: Vector2<f32>) -> Vector2<f32> {
        // Part of this transfrom is a scaling operation. We can figure this out by figuring out
        // the radius of the circle in pixels that will map to the radius of the unit circle
        // The radius is either half of self.window_width or window_height depending on which is
        // smaller
        let pixel_radius = (if self.window_width >= self.window_height {
            self.window_height
        } else {
            self.window_width
        }) / 2.0;

        // The other part of the transform is a translation. The origin in mouse coordinates is the
        // top left corner of the screen. In screen space its the center of the screen.
        // So we are going to need to know the screen center in mouse space
        let screen_center = Vector2::new(self.window_width, self.window_height) * 0.5;

        // Translate point then scale
        let mut screen_point = (mouse_coords - screen_center) / pixel_radius;

        // The last part of the transform is inverting the y-axis, since the mouse y-axis and the
        // screen space y-axis are inverted
        screen_point.y *= -1.0;

        screen_point
    }

    // The ArcBall controls work by mapping points in screen space onto the unit circle
    // circumscribed by screen space, and then mapping points from that circle on the unit sphere.
    // In this way, two points on unit sphere can be used to define a rotation.
    fn mouse_to_sphere_point(&self, mouse_coords: Vector2<f32>) -> Vector3<f32> {
        let screen_point = self.mouse_to_screen(mouse_coords);

        // Now we find point on sphere by clamping to unit circle
        // and finding z component
        let screen_point_radius_squared = screen_point.magnitude2();
        let sphere_point = if screen_point_radius_squared >= 1.0 {
            // Points on, or mapped to, the circle itself have no z component
            (screen_point / screen_point_radius_squared.sqrt()).extend(0.0)
        } else {
            // Points in the circle get "pushed onto" the sphere
            // The rotation axis extends into the screen, hence the negative
            screen_point.extend(-(1.0 - screen_point_radius_squared).sqrt())
        };

        // If we were contraining axis, that would go here

        sphere_point
    }

    // When panning we want to the mouse to act like it was dragging the camera target around
    // That means we need to map the screen space on the plane that is camera.distance away and
    // orthogronal to the viewing direction of the camera
    fn mouse_to_pan_point(&self, mouse_coords: Vector2<f32>) -> Vector3<f32> {
        let screen_point = self.mouse_to_screen(mouse_coords);

        // Using similiar triangles we can scale the screen point onto a plane camera.distance away
        // since we know that the "distance" to the screen plane is defined by the near attribute
        // of our viewing frustrum
        //
        // point_distance / point_screen = distance / near
        // =>
        // point_distance = point_screen * (disance / near)
        let near = perspective::fov_near_distance(self.field_of_view);
        let distance_plane_point = (screen_point * (self.distance / near)).extend(self.distance);

        // Then we need to rotate that point to so that it matches the direction our camera is
        // facing
        let pan_point = Matrix3::from(self.get_rotation()) * distance_plane_point;

        pan_point
    }

    /// Handle mouse movement as pixel coordinates
    pub fn handle_mouse_move(&mut self, mouse_x: f32, mouse_y: f32) {
        self.prev_mouse_coords = Vector2::new(mouse_x, mouse_y);

        match self.state {
            CamState::Tumble => {
                // This is the Arcball movement
                // The original and new sphere point define a rotation
                // so we convert that into a quaternion and update the camera's rotation
                let sphere_point = self.mouse_to_sphere_point(self.prev_mouse_coords);
                let rotation_axis = self.original_sphere_point.cross(sphere_point);
                let scalar = self.original_sphere_point.dot(sphere_point);
                let move_rotation = Quaternion::from_sv(scalar, rotation_axis);
                let new_rotation = self.original_rotation * move_rotation;
                self.rotation = new_rotation;
            }
            CamState::Pan => {
                // The original and new pan point define a translation
                let pan_point = self.mouse_to_pan_point(self.prev_mouse_coords);
                let pan_delta = self.original_pan_point - pan_point;
                self.target = self.original_target + pan_delta;
            }
            _ => (),
        }
    }

    pub fn toggle_orbit(&mut self) {
        self.orbit_enabled = !self.orbit_enabled;
    }

    /// Handle mouse clicks,
    pub fn handle_mouse_input(&mut self, button: MouseButton, state: ButtonState) {
        match (button, state) {
            (MouseButton::Left, ButtonState::Pressed) => {
                self.state = CamState::Tumble;
                self.tumble_duration = 0.0;
                self.original_sphere_point = self.mouse_to_sphere_point(self.prev_mouse_coords);
                self.original_rotation = self.rotation.clone();
            }
            (MouseButton::Right, ButtonState::Pressed) => {
                self.state = CamState::Pan;
                self.original_pan_point = self.mouse_to_pan_point(self.prev_mouse_coords);
                self.original_target = self.target.clone();
            }
            (_, ButtonState::Released) => {
                if self.orbit_enabled && self.state == CamState::Tumble {
                    self.state = CamState::IdleOrbit;

                //let angle_delta= self.last_tumble_delta.s.cos() * 2.0;
                //let angle_delta = new_angle - old_angle;
                //self.orbit_velocity = (angle_delta * 1000.0) / self.tumble_duration;
                } else {
                    self.state = CamState::Idle;
                }
            }
        }
    }

    // Handle scroll events as pixel deltas
    pub fn handle_scroll(&mut self, pixel_delta: f32) {
        let normalized_delta = pixel_delta * self.scroll_modifier;

        let scale = 1.0 + normalized_delta;

        self.distance *= scale;
    }

    /// Move the camera's target
    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.target += delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert::*;
    use cgmath::vec2;
    use std::default::Default;
    use std::f32;

    fn make_cam_with_window(window_width: f32, window_height: f32) -> Camera {
        let mut camera = Camera::new();
        camera.update(Default::default(), window_width, window_height);
        camera
    }

    #[test]
    fn test_mouse_to_square_screen() {
        let camera = make_cam_with_window(1024.0, 1024.0);
        let mut screen_point: [f32; 2];

        // Top Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 0.0)).into();
        close(&screen_point, &[-1.0, 1.0], f32::EPSILON);

        // Top Rigth
        screen_point = camera.mouse_to_screen(vec2(1024.0, 0.0)).into();
        close(&screen_point, &[1.0, 1.0], f32::EPSILON);

        // Bottom Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 1024.0)).into();
        close(&screen_point, &[-1.0, -1.0], f32::EPSILON);

        // Bottom Right
        screen_point = camera.mouse_to_screen(vec2(1024.0, 1024.0)).into();
        close(&screen_point, &[1.0, -1.0], f32::EPSILON);

        // Center
        screen_point = camera.mouse_to_screen(vec2(512.0, 512.0)).into();
        close(&screen_point, &[0.0, 0.0], f32::EPSILON);
    }

    #[test]
    fn test_mouse_to_wide_screen() {
        let camera = make_cam_with_window(1024.0, 512.0);
        let mut screen_point: [f32; 2];

        // Top Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 0.0)).into();
        close(&screen_point, &[-2.0, 1.0], f32::EPSILON);

        // Top Rigth
        screen_point = camera.mouse_to_screen(vec2(1024.0, 0.0)).into();
        close(&screen_point, &[2.0, 1.0], f32::EPSILON);

        // Bottom Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 512.0)).into();
        close(&screen_point, &[-2.0, -1.0], f32::EPSILON);

        // Bottom Right
        screen_point = camera.mouse_to_screen(vec2(1024.0, 512.0)).into();
        close(&screen_point, &[2.0, -1.0], f32::EPSILON);

        // Center
        screen_point = camera.mouse_to_screen(vec2(512.0, 256.0)).into();
        close(&screen_point, &[0.0, 0.0], f32::EPSILON);
    }

    #[test]
    fn test_mouse_to_tall_screen() {
        let camera = make_cam_with_window(512.0, 1024.0);
        let mut screen_point: [f32; 2];

        // Top Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 0.0)).into();
        close(&screen_point, &[-1.0, 2.0], f32::EPSILON);

        // Top Rigth
        screen_point = camera.mouse_to_screen(vec2(512.0, 0.0)).into();
        close(&screen_point, &[1.0, 2.0], f32::EPSILON);

        // Bottom Left
        screen_point = camera.mouse_to_screen(vec2(0.0, 1024.0)).into();
        close(&screen_point, &[-1.0, -2.0], f32::EPSILON);

        // Bottom Right
        screen_point = camera.mouse_to_screen(vec2(512.0, 1024.0)).into();
        close(&screen_point, &[1.0, -2.0], f32::EPSILON);

        // Center
        screen_point = camera.mouse_to_screen(vec2(256.0, 512.0)).into();
        close(&screen_point, &[0.0, 0.0], f32::EPSILON);
    }

}
