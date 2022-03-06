extern crate image;

use std::f64::consts::PI;

use image::io::Reader as ImageReader;
use ray_tracing::color::*;
use ray_tracing::math::*;
use ray_tracing::scene::*;

fn test() -> Scene {
    let mut scene = Scene {
        width: 2000,
        height: 2000,
        fov: 90.,
        shadow_bias: 1e-6,
        max_recursion_depth: 100,
        lights: vec![],
        elements: vec![],
    };

    let lights: Vec<Light> = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: 2.0,
                y: 0.,
                z: -3.,
            },
            color: Color {
                red: 0.8,
                green: 1.,
                blue: 0.8,
            },
            intensity: 300.,
        }),
        Light::Directional(DirectionalLight {
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            },
            direction: Vector3 {
                x: -0.25,
                y: -1.,
                z: -1.,
            },
            intensity: 20.,
        }),
    ];

    let colors = [
        Color {
            red: 0.2,
            green: 1.0,
            blue: 0.2,
        },
        Color {
            red: 1.,
            green: 0.2,
            blue: 0.2,
        },
        Color {
            red: 0.2,
            green: 0.2,
            blue: 1.,
        },
    ];
    let iterations = 15;
    for n in 1..(iterations + 1) {
        let checkerboard_image = ImageReader::open("checkerboard.png")
            .ok()
            .unwrap()
            .decode()
            .ok()
            .unwrap();
        let texture = Coloration::Texture(checkerboard_image);
        let phi = ((2 * n) as f64) * PI / (iterations as f64);

        let r = 1.;
        let c = [
            phi.cos() * r,
            phi.sin() * r,
            -2.8 + (n as f64) / (iterations as f64),
        ];
        let p = Point::from_list(c);

        let current_color = colors[n % 3];

        let green_sphere = Element::Sphere(Sphere {
            center: p,
            radius: 0.5 * (n as f64) / (iterations as f64),
            material: Material {
                surface_type: SurfaceType::Reflective { reflectivity: 0.2 },
                // color: texture,
                color: Coloration::Color(current_color),
                albedo: 0.18,
            },
        });
        scene.add_element(green_sphere);
    }

    for light in lights {
        scene.add_light(light);
    }

    let checkerboard_image = ImageReader::open("checkerboard.png")
        .ok()
        .unwrap()
        .decode()
        .ok()
        .unwrap();
    let texture = Coloration::Texture(checkerboard_image);

    let bottom_plane = Element::Plane(Plane {
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
            surface_type: SurfaceType::Reflective { reflectivity: 0.3 },
            color: texture,
            albedo: 0.18,
        },
    });

    scene.add_element(bottom_plane);

    scene
}

fn default_scene() -> Scene {
    let checkerboard_image = ImageReader::open("checkerboard.png")
        .ok()
        .unwrap()
        .decode()
        .ok()
        .unwrap();

    let texture = Coloration::Texture(checkerboard_image);

    let mut scene = Scene {
        width: 800,
        height: 600,
        fov: 90.,
        shadow_bias: 1e-6,
        max_recursion_depth: 20,
        lights: vec![],
        elements: vec![],
    };

    let mut lights = vec![
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
                x: 0.025,
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
                x: 2.0,
                y: 0.,
                z: -3.,
            },
            color: Color {
                red: 0.8,
                green: 1.,
                blue: 0.8,
            },
            intensity: 300.,
        }),
    ];

    let green_sphere = Element::Sphere(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -4.,
        },
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

    let blue_sphere = Element::Sphere(Sphere {
        center: Point {
            x: -3.0,
            y: 1.0,
            z: -6.0,
        },
        radius: 2.,
        material: Material {
            surface_type: SurfaceType::Reflective { reflectivity: 0.1 },
            color: Coloration::Color(Color {
                red: 0.2,
                green: 0.2,
                blue: 1.,
            }),
            albedo: 0.58,
        },
    });

    let red_sphere = Element::Sphere(Sphere {
        center: Point {
            x: 2.7,
            y: 1.5,
            z: -5.0,
        },
        radius: 2.0,
        material: Material {
            surface_type: SurfaceType::Reflective { reflectivity: 0.1 },
            color: Coloration::Color(Color {
                red: 1.,
                green: 0.2,
                blue: 0.2,
            }),
            albedo: 0.08,
        },
    });

    let bottom_plane = Element::Plane(Plane {
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
            surface_type: SurfaceType::Reflective { reflectivity: 0.3 },
            color: texture,
            albedo: 0.18,
        },
    });

    let back_plane = Element::Plane(Plane {
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
            surface_type: SurfaceType::Diffuse,
            color: Coloration::Color(Color {
                red: 0.6,
                green: 0.8,
                blue: 1.0,
            }),
            albedo: 0.18,
        },
    });

    while !lights.is_empty() {
        scene.add_light(lights.pop().unwrap());
    }

    scene.add_element(blue_sphere);
    scene.add_element(red_sphere);
    scene.add_element(green_sphere);
    scene.add_element(bottom_plane);
    scene.add_element(back_plane);

    scene
}

pub fn main() {
    // let scene = default_scene();
    let scene = test();
    let dyn_image = ray_tracing::render(&scene);

    dyn_image.save("test.png").unwrap();
}
