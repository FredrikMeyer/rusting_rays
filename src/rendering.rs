#[cfg(test)]
use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::scene::{Scene, Sphere};

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
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
        Intersection { distance, object }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ray_origin = ray.origin;
        let ray_direction = &ray.direction;
        let sphere_center = self.center;

        // From origin to sphere center
        let l = sphere_center - ray_origin;
        let hyp_sq = l.sq_length();

        let a = l.dot(&ray_direction);

        let d2 = hyp_sq - a * a;
        let r2 = self.radius * self.radius;

        if d2 > r2 {
            return None;
        }

        let q2 = r2 - d2;

        // TODO: handle the case when the sphere is behind the camera
        // See here https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection

        Some((a - q2).sqrt())
    }
}

#[test]
fn test_intersect() {
    let sphere = Sphere {
        center: Point {
            x: 1.,
            y: 0.,
            z: 0.,
        },
        radius: 0.5,
        color: Color {
            red: 0.,
            green: 0.,
            blue: 0.,
        },
        albedo: 0.17,
    };

    let ray = Ray {
        direction: Vector3 {
            x: 1.,
            y: 0.,
            z: 0.,
        },
        origin: Point::zero(),
    };

    let ray2 = Ray {
        direction: Vector3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        origin: Point::zero(),
    };

    assert!(sphere.intersect(&ray).is_some());
    assert!(sphere.intersect(&ray).unwrap() > 0.);
    assert!(sphere.intersect(&ray2).is_none());
}
