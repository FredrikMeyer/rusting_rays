extern crate image;

pub mod color;
pub mod math;
mod rendering;
pub mod scene;
#[cfg(test)]
pub mod test;
pub mod test_scene;

use color::Color;
use color::BLACK;
use image::{DynamicImage, GenericImage};
use math::Vector3;
use rendering::Ray;
use scene::Intersectable;
use scene::Intersection;
use scene::Light;
use scene::Scene;
use scene::SurfaceType;
use scene::{Coloration, Element, Material, Sphere};

use js_sys;
use log::{info, Level};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Couldnt init logger");
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageRawData {
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct JSPoint {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct TestOptions {}

#[wasm_bindgen]
impl TestOptions {
    pub fn new() -> () {}
}

#[wasm_bindgen]
impl ImageRawData {
    pub fn get_image(width: u32, height: u32, point: JsValue) -> ImageRawData {
        // serde_wasm_bindgen::from_value(value).ex
        let point: JSPoint = serde_wasm_bindgen::from_value(point).expect("hmm");

        info!("{:?}", point);
        get_image_data(width, height, point)
    }

    #[wasm_bindgen]
    pub fn get_data(&self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view(&self.data[..]) }
    }

    pub fn get_width(&self) -> js_sys::Number {
        js_sys::Number::from(self.width as u32)
    }

    pub fn get_height(&self) -> js_sys::Number {
        js_sys::Number::from(self.height as u32)
    }
}

pub fn get_image_data(width: u32, height: u32, point: JSPoint) -> ImageRawData {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let w = width;
    let h = height;
    let mut data = Vec::with_capacity((w * h) as usize);
    for i in 0..w {
        for j in 0..h {
            data.push((i * 255 / w) as u8);
            data.push((j * 255 / w) as u8);
            data.push((i * 255 / w) as u8);
            data.push(255 as u8);
        }
    }

    let mut scene = test_scene::test_scene(width, height);
    let world_coord = scene.pixel_to_world_coordinates(point.x as u32, point.y as u32, -3.0);
    let sphere = Element::Sphere(Sphere {
        center: world_coord,
        radius: 1.0,
        material: Material {
            surface_type: SurfaceType::Reflective { reflectivity: 0.2 },
            color: Coloration::Color(Color {
                red: 0.2,
                green: 1.0,
                blue: 0.2,
            }),
            albedo: 0.18,
        },
    });
    scene.add_element(sphere);
    render_to_image_data(&scene)
}

pub fn render_to_image_data(scene: &Scene) -> ImageRawData {
    let w = scene.width;
    let h = scene.height;
    let mut data = Vec::<u8>::with_capacity((w * h * 4) as usize);
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::create_prime(x, y, scene);

            let mut color = cast_ray(scene, &ray, 0).to_vec();
            data.append(&mut color)
        }
    }

    ImageRawData {
        data,
        width: w as usize,
        height: h as usize,
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let color = cast_ray(scene, &ray, 0);
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return BLACK;
    }

    let intersection = scene.trace(&ray);

    intersection
        .map(|i| get_color(scene, ray, &i, depth))
        .unwrap_or(BLACK)
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point: Vector3 = ray.origin.as_vector() + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point.as_point());

    let mut color = shade_diffuse(scene, intersection.object, hit_point, surface_normal);

    if let SurfaceType::Reflective { reflectivity } = intersection.object.material().surface_type {
        let reflection_ray = Ray::create_reflection(
            surface_normal,
            &ray.direction,
            &hit_point.as_point(),
            scene.shadow_bias,
        );
        color = color * (1.0 - reflectivity);
        color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    }

    color
}

pub fn shade_diffuse(
    scene: &Scene,
    element: &Element,
    hit_point: Vector3,
    surface_normal: Vector3,
) -> Color {
    let texture_coords = element.texture_coords(&hit_point.as_point());
    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    for light in scene.lights.iter() {
        let light_color = match light {
            Light::Directional(l) => {
                let direction_to_light = l.direction.normalize() * -1.; // minus??

                let shadow_ray = Ray {
                    origin: (hit_point + surface_normal * 1e-6).as_point(),
                    direction: direction_to_light,
                };
                let in_light = scene.trace(&shadow_ray).is_none();

                let light_intensity = if in_light { l.intensity } else { 0.0 };
                let light_power =
                    (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
                let light_reflected = element.albedo() / std::f32::consts::PI;

                l.color * light_power * light_reflected
            }
            Light::Spherical(l) => {
                let direction_to_light = l.position - hit_point.as_point();

                let distance = direction_to_light.length() as f32;
                let intensity = l.intensity / (4.0 * std::f32::consts::PI * distance * distance);

                let shadow_ray = Ray {
                    origin: (hit_point + surface_normal * 1e-6).as_point(),
                    direction: direction_to_light.normalize(),
                };

                let shadow_intersection = scene.trace(&shadow_ray);
                let in_light = shadow_intersection.is_none()
                    || shadow_intersection.unwrap().distance
                        > l.position.distance(&hit_point.as_point());

                let light_intensity = if in_light { intensity } else { 0.0 };
                let light_power =
                    (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
                let light_reflected = element.albedo() / std::f32::consts::PI;

                l.color * light_power * light_reflected
            }
        };

        color = color + element.material().color.color(&texture_coords) * (light_color);
    }

    color.clamp()
}
