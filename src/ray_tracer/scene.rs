use std::f32;

use super::image::Image;
use super::light::Light;
use super::material::Material;
use super::model::Model;
use super::plane::Plane;
use super::sphere::Sphere;
use super::Vec3f;

pub struct Scene<'a> {
    pub environment_map: Image,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere<'a>>,
    pub model: Model<'a>,
    pub plane: Plane,
}

impl<'a> Scene<'a> {
    pub fn background_color(&self, dir: &Vec3f) -> Vec3f {
        let pi = f32::consts::PI;
        let env = &self.environment_map;
        let (x, y, z) = (dir[0], dir[1], dir[2]); //slice patterns are unstable
        let r = dir.norm();
        let mut phi = (z / x).atan();
        if x < 0. {
            phi += pi;
        };
        let theta = (y / r).acos();
        let x = (env.width as f32 * (phi / pi + 1.) / 2.) as usize;
        let y = (env.height as f32 * (theta / pi)) as usize;
        env.buffer[y * env.width + x]
    }

    pub fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<(Vec3f, Vec3f, Material)> {
        let (mut dist, mut point) = spheres_intersect(orig, dir, &self.spheres);
        let (board_dist, board_point) = self.plane.intersect(orig, dir);
        if board_dist < dist {
            dist = board_dist;
            point = board_point;
        }
        let (model_dist, model_point) = self.model.intersect(orig, dir);
        if model_dist < dist {
            dist = model_dist;
            point = model_point;
        }

        if dist > 1000. {
            None
        } else {
            Some(point)
        }
    }
}

fn spheres_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
) -> (f32, (Vec3f, Vec3f, Material)) {
    let mut dist = f32::MAX;
    let mut hit_point = Vec3f::zeros();
    let mut n = Vec3f::zeros();
    let mut material = Material::default();

    for sphere in spheres {
        if let Some(dist_i) = sphere.ray_intersect(orig, dir) {
            if dist_i >= dist {
                continue;
            }
            dist = dist_i;
            hit_point = orig + dir * dist;
            let delta = hit_point - sphere.center;
            n = delta.normalize();
            material = sphere.material.clone();
        }
    }
    (dist, (hit_point, n, material))
}
