#![warn(clippy::all)]

mod ray_tracer;

use ray_tracer::*;

fn main() -> std::io::Result<()> {
    let environment_map = Image::load("envmap.jpg");
    let duck = Geometry::load("duck.obj");

    let ivory = Material {
        refractive_index: 1.0,
        albedo: Vec4f::new(0.6, 0.3, 0.1, 0.0),
        diffuse_color: Vec3f::new(0.4, 0.4, 0.3),
        specular_exponent: 50.,
    };
    let glass = Material {
        refractive_index: 1.5,
        albedo: Vec4f::new(0.0, 0.5, 0.1, 0.8),
        diffuse_color: Vec3f::new(0.6, 0.7, 0.8),
        specular_exponent: 125.,
    };
    let red_rubber = Material {
        refractive_index: 1.0,
        albedo: Vec4f::new(0.9, 0.1, 0.0, 0.0),
        diffuse_color: Vec3f::new(0.3, 0.1, 0.1),
        specular_exponent: 10.,
    };
    let mirror = Material {
        refractive_index: 1.0,
        albedo: Vec4f::new(0.0, 10., 0.8, 0.0),
        diffuse_color: Vec3f::new(1.0, 1.0, 1.0),
        specular_exponent: 1425.,
    };

    let spheres = vec![
        Sphere {
            center: Vec3f::new(-3.0, 0.0, -16.),
            radius: 2.,
            material: &ivory,
        },
        Sphere {
            center: Vec3f::new(-1.0, -1.5, -12.),
            radius: 2.,
            material: &glass,
        },
        Sphere {
            center: Vec3f::new(1.5, -0.5, -18.),
            radius: 3.,
            material: &red_rubber,
        },
        Sphere {
            center: Vec3f::new(7.0, 5.0, -18.),
            radius: 4.,
            material: &mirror,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3f::new(-20., 20., 20.),
            intensity: 1.5,
        },
        Light {
            position: Vec3f::new(30., 50., -25.),
            intensity: 1.8,
        },
        Light {
            position: Vec3f::new(30., 20., 30.),
            intensity: 1.7,
        },
    ];

    let plane = Plane {
        material: Default::default(),
    };

    let scene = Scene {
        environment_map,
        lights,
        spheres,
        model: Model {
            geometry: duck,
            material: &glass,
        },
        plane,
    };

    let render = Render {
        max_depth: 4,
        scene,
    };

    render.get_image().save()
}
