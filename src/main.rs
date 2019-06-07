extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use na::{Vector3, UnitQuaternion};
use kiss3d::event::{Action, WindowEvent};
use kiss3d::window::Window;
use kiss3d::light::Light;
use rand::random;

fn main() {
    let mut window = Window::new("kiss3d-rotating-color-changing-cube");
    let mut c = window.add_cube(0.3, 0.3, 0.3);

    c.set_color(random(), random(), random());

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::MouseButton(_button, Action::Release, _mods) => {
                   c.set_color(random(), random(), random()); 
                }
                _ => {}
            }
        }

        c.prepend_to_local_rotation(&rot);
    }
}
