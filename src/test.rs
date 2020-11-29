mod tests {
    use image::DynamicImage;
    use image::GenericImageView;

    use crate::color::Color;
    use crate::math::Point;
    use crate::math::Vector3;
    use crate::render;
    use crate::rendering::Ray;
    use crate::scene::Coloration;
    use crate::scene::DirectionalLight;
    use crate::scene::Element;
    use crate::scene::Intersectable;
    use crate::scene::Light;
    use crate::scene::Material;
    use crate::scene::Plane;
    use crate::scene::SurfaceType;
    use crate::scene::{Scene, Sphere};

    #[test]
    fn test_can_render_scene() {
        let scene = Scene {
            width: 80,
            height: 60,
            max_recursion_depth: 3,
            shadow_bias: 1e-6,
            fov: 90.0,
            lights: vec![Light::Directional(DirectionalLight {
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
            })],
            elements: vec![Element::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                material: Material {
                    surface_type: SurfaceType::Diffuse,
                    color: Coloration::Color(Color {
                        red: 0.4,
                        green: 1.0,
                        blue: 0.4,
                    }),
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
        let sphere = Sphere {
            center: Point {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            radius: 0.5,
            material: Material {
                surface_type: SurfaceType::Diffuse,
                color: Coloration::Color(Color {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                }),
                albedo: 0.17,
            },
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
            material: Material {
                surface_type: SurfaceType::Diffuse,
                color: Coloration::Color(Color {
                    red: 0.6,
                    green: 0.8,
                    blue: 1.0,
                }),
                albedo: 0.18,
            },
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
}
