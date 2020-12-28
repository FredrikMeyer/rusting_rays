use image::DynamicImage;
use image::GenericImageView;

use crate::color::Color;
use crate::math::Point;
use crate::math::Vector3;
use crate::rendering::Ray;

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

// pub struct Texture {
//     pub path: PathBuf,

// }

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage),
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match &self {
            Coloration::Color(c) => *c,
            Coloration::Texture(texture) => {
                let tex_x = wrap(texture_coords.x, texture.width());
                let tex_y = wrap(texture_coords.y, texture.height());

                Color::from_rgba(&texture.get_pixel(tex_x, tex_y))
            }
        }
    }
}

pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub surface_type: SurfaceType,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    // Color stuff, move to impl or something?
    pub material: Material,
}

pub struct Plane {
    pub p0: Point,
    pub normal: Vector3,
    pub material: Material,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
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

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
        Intersection { distance, object }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
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

    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn new(
        width: u32,
        height: u32,
        fov: f64,
        shadow_bias: f64,
        max_recursion_depth: u32,
    ) -> Scene {
        Scene {
            width,
            height,
            fov,
            lights: vec![],
            elements: vec![],
            shadow_bias,
            max_recursion_depth,
        }
    }

    pub fn pixel_to_world_coordinates(&self, px: u32, py: u32, z: f64) -> Point {
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let sensor_x =
            ((((px as f64 + 0.5) / self.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((py as f64 + 0.5) / self.height as f64) * 2.0) * fov_adjustment;

        Point {
            x: sensor_x,
            y: sensor_y,
            z,
        }
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
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

    fn surface_normal(&self, point: &Point) -> Vector3 {
        (*point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / std::f32::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }
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

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        self.normal * -1.
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 1.,
            y: 0.,
            z: 0.,
        });

        if x_axis.length() == 0. {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.,
                y: 1.,
                z: 0.,
            });
        }

        let y_axis = x_axis.cross(&self.normal);

        let point_as_vector = hit_point.as_vector();

        TextureCoords {
            x: point_as_vector.dot(&x_axis) as f32,
            y: point_as_vector.dot(&y_axis) as f32,
        }
    }
}
