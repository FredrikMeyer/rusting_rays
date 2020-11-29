extern crate image;

use ray_tracing::color::*;
use ray_tracing::math::*;
use ray_tracing::scene::*;

pub fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: -0.25,
                    y: -1.,
                    z: -1.,
                },
                color: Color {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                intensity: 20.,
            }),
            Light::Directional(DirectionalLight {
                direction: Vector3 {
                    x: 0.25,
                    y: 1.,
                    z: -1.,
                },
                color: Color {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                intensity: 20.,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: -2.,
                    y: 10.,
                    z: -3.,
                },
                color: Color {
                    red: 0.8,
                    green: 1.,
                    blue: 0.8,
                },
                intensity: 300.,
            }),
        ],
        elements: vec![
            Element::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                material: Material {
                    color: Color {
                        red: 0.2,
                        green: 1.0,
                        blue: 0.2,
                    },
                    albedo: 0.18,
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: -3.0,
                    y: 1.0,
                    z: -6.0,
                },
                radius: 2.,
                material: Material {
                    color: Color {
                        red: 0.2,
                        green: 0.2,
                        blue: 1.,
                    },
                    albedo: 0.58,
                },
            }),
            Element::Sphere(Sphere {
                center: Point {
                    x: 2.,
                    y: 2.,
                    z: -4.0,
                },
                radius: 2.25,
                material: Material {
                    color: Color {
                        red: 1.,
                        green: 0.2,
                        blue: 0.2,
                    },
                    albedo: 0.08,
                },
            }),
            Element::Plane(Plane {
                normal: Vector3 {
                    x: 0.,
                    y: -1.,
                    z: 0.0,
                },
                p0: Point {
                    x: 0.,
                    y: -2.,
                    z: 0.,
                },
                material: Material {
                    color: Color {
                        red: 0.2,
                        green: 0.2,
                        blue: 0.2,
                    },
                    albedo: 0.18,
                },
            }),
            Element::Plane(Plane {
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
                material: Material {
                    color: Color {
                        red: 0.6,
                        green: 0.8,
                        blue: 1.0,
                    },
                    albedo: 0.18,
                },
            }),
        ],
    };

    let dyn_image = ray_tracing::render(&scene);

    dyn_image.save("test.png").unwrap();
}
