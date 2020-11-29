extern crate image;

pub mod color;
pub mod math;
mod rendering;
pub mod scene;

use color::Color;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use math::{Point, Vector3};
use rendering::{Intersectable, Ray};
use scene::Element;
use scene::Intersection;
use scene::Light;
use scene::Material;
use scene::SphericalLight;
use scene::{Scene, Sphere};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    let black = Rgba::from_channels(0, 0, 0, 0);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            match scene.trace(&ray) {
                Some(intersection) => {
                    // let color = intersection.object.color().clone();

                    let color = get_color(scene, &ray, &intersection);

                    image.put_pixel(x, y, color.clamp().to_rgba());
                }
                None => {
                    image.put_pixel(x, y, black);
                }
            }
        }
    }
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point: Vector3 = ray.origin.as_vector() + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.normal(&hit_point.as_point());

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
                let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

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
                let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

                l.color * light_power * light_reflected
            }
        };

        color = color + *intersection.object.color() * (light_color);
    }

    color.clamp()
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 80,
        height: 60,
        fov: 90.0,
        lights: vec![Light {
            direction: Vector3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            },
            intensity: 100.,
        }],
        elements: vec![Element::Sphere(Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            material: Material {
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
                albedo: 0.18,
            },
        })],
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

#[test]
fn test_intersect() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        lights: vec![Light {
            direction: Vector3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            },
            intensity: 100.,
        }],
        elements: vec![Element::Sphere(Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            material: Material {
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
                albedo: 0.17,
            },
        })],
    };

    let ray = Ray::create_prime(400, 300, &scene);
    println!("{:?}", ray);
    assert!(scene.trace(&ray).is_some());
    let ray2 = Ray::create_prime(0, 0, &scene);
    assert!(scene.trace(&ray2).is_none());
}
