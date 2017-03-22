extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c      = window.add_cube(1.0, 1.0, 1.0);
    let eye        = Point3::new(0.0, 0.0, -3.0); // stand way back
    let at         = Point3::new(0.0, 0.0, 0.0); // point at middle of scene (cube?)
    let mut camera = ArcBall::new(eye, at);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render_with_camera(&mut camera) {
        c.prepend_to_local_rotation(&rot);
    }
}
