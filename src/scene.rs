use crate::color::Color;
use crate::math::Point;
use crate::math::Vector3;
use crate::rendering::Ray;

pub struct Material {
    pub color: Color,
    pub albedo: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    // Color stuff, move to impl or something?
    pub material: Material,
}

impl Sphere {
    pub fn surface_normal(&self, point: &Point) -> Vector3 {
        (*point - self.center).normalize()
    }
}

pub struct Plane {
    pub p0: Point,
    pub normal: Vector3,
    // Color stuff
    pub material: Material,
}

impl Plane {
    pub fn surface_normal(&self, _: &Point) -> Vector3 {
        // Bloggposten har en minus her??
        self.normal * -1.
    }
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.material.color,
            Element::Plane(ref p) => &p.material.color,
        }
    }

    pub fn normal(&self, point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(point),
            Element::Plane(ref p) => p.surface_normal(point),
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub lights: Vec<Light>,
    pub elements: Vec<Element>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("infinite dinstance");
        }
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

        let q = q2.sqrt();
        let t0 = a - q;
        let t1 = a + q;

        if t0 < 0. && t1 < 0. {
            return None;
        }

        Some(t0.min(t1))

        // TODO: handle the case when the sphere is behind the camera
        // See here https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection

        // Some(a - q2.sqrt())
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let n = self.normal;
        let r = ray.direction;

        if n.dot(&r).abs() < 1e-6 {
            return None;
        }

        let h = self.p0 - ray.origin;

        let dist = (n.dot(&h)) / n.dot(&r);

        if dist >= 0. {
            return Some(dist);
        }
        None
    }
}

#[test]
fn test_plane_intersect() {
    let plane = Plane {
        normal: Vector3 {
            x: 0.,
            y: 0.,
            z: -1.0,
        },
        p0: Point {
            x: 0.,
            y: 0.,
            z: -20.,
        },
        color: Color {
            red: 0.6,
            green: 0.8,
            blue: 1.0,
        },
        albedo: 0.18,
    };

    let ray = Ray {
        direction: Vector3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        origin: Point::zero(),
    };

    let ray2 = Ray {
        direction: Vector3 {
            x: 1.,
            y: 1.,
            z: -1.,
        },
        origin: Point::zero(),
    };

    let intersection = plane.intersect(&ray);
    let intersection2 = plane.intersect(&ray2);

    assert!(intersection.is_some());
    assert!(intersection2.is_some());
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
        albedo: 0.18,
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
