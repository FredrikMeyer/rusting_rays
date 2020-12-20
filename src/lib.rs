extern crate image;

pub mod color;
pub mod math;
mod rendering;
pub mod scene;
#[cfg(test)]
pub mod test;

use color::Color;
use color::BLACK;
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use math::Vector3;
use rendering::Ray;
use scene::Element;
use scene::Intersectable;
use scene::Intersection;
use scene::Light;
use scene::Scene;
use scene::SurfaceType;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    let black = Rgba::from_channels(0, 0, 0, 0);

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
