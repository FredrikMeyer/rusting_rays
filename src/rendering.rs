use crate::math::{Point, Vector3};
use crate::scene::Scene;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    /// Should always create a normalized vector
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width >= scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }

    pub fn create_reflection(
        normal: Vector3,
        incident: &Vector3,
        intersection: &Point,
        bias: f64,
    ) -> Ray {
        Ray {
            origin: (intersection.as_vector() + normal * bias).as_point(),
            direction: *incident - (normal * incident.dot(&normal) * 2.0),
        }
    }

    pub fn create_transmission(
        normal: Vector3,
        incident: Vector3,
        intersection: &Point,
        bias: f64,
        index: f32,
    ) -> Option<Ray> {
        let mut ref_n = normal;
        let mut eta_t = index as f64;
        let mut eta_i = 1.0f64;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            ref_n = normal * -1.;
            eta_t = 1.0;
            eta_i = index as f64;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: (intersection.as_vector() + (ref_n * -bias)).as_point(),
                direction: (incident + ref_n * i_dot_n) * eta - ref_n * k.sqrt(),
            })
        }
    }
}
