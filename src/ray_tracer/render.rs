use std::f32;

use rayon::prelude::*;

use super::raster::Image;
use super::scene::Scene;
use super::Vec3f;

pub struct Render<'a> {
    pub max_depth: usize,
    pub scene: Scene<'a>,
}

fn reflect(i: &Vec3f, n: &Vec3f) -> Vec3f {
    i - n * 2. * i.dot(n)
}

fn refract(&i: &Vec3f, n: &Vec3f, eta_t: f32, eta_i: f32) -> Option<Vec3f> {
    let cos_i = -(i.dot(&n));

    if cos_i < 0. {
        return refract(&i, &(-n), eta_i, eta_t); // if the ray comes from the inside the object, swap the air and the media
    }

    let eta = eta_i / eta_t;
    let sin2_i = 1. - cos_i * cos_i;
    let sin2_t = eta * eta * sin2_i;

    if sin2_t > 1.0 {
        None
    } else {
        let cos_t = (1. - sin2_t).sqrt();
        Some(i * eta + n * (-cos_t + eta * cos_i))
    }
}

impl<'a> Render<'a> {
    fn cast_ray(&self, orig: &Vec3f, dir: &Vec3f, depth: usize) -> Vec3f {
        if depth > self.max_depth {
            return Vec3f::zeros();
        }
        if let Some((point, n, material)) = self.scene.intersect(orig, dir) {
            let reflect_dir = reflect(dir, &n);
            let dn = n * 1e-3;
            // offset the original point to avoid occlusion by the object itself
            let reflect_orig = if reflect_dir.dot(&n) < 0. {
                point - dn
            } else {
                point + dn
            };
            let reflect_color = self.cast_ray(&reflect_orig, &reflect_dir, depth + 1);

            let refract_color = if let Some(rdir) = refract(dir, &n, material.refractive_index, 1.0)
            {
                let refract_dir = rdir.normalize();
                let refract_orig = if refract_dir.dot(&n) < 0. {
                    point - dn
                } else {
                    point + dn
                };
                if n[2] <= -1.999 {
                    dbg!(refract_dir);
                    dbg!(dir);
                    dbg!(n);
                    dbg!(point);
                    dbg!(refract_orig);
                }
                self.cast_ray(&refract_orig, &refract_dir, depth + 1)
            } else {
                Vec3f::zeros()
            };

            let mut diffuse_light_intensity = 0f32;
            let mut specular_light_intensity = 0f32;

            for light in &self.scene.lights {
                let light_dir = (light.position - point).normalize();
                let shadow_orig = if light_dir.dot(&n) < 0. {
                    point - dn
                } else {
                    point + dn
                };
                let light_distance = (light.position - point).norm();

                let light_ray_dir = (light.position - point).normalize();

                // checking if the point lies in the shadow of the lights[i]
                if let Some((shadow_pt, _, _)) = self.scene.intersect(&shadow_orig, &light_ray_dir)
                {
                    if (shadow_pt - shadow_orig).norm() <= light_distance {
                        continue;
                    }
                }

                diffuse_light_intensity += light.intensity * 0f32.max(light_dir.dot(&n));

                let specular = 0f32.max((-reflect(&(-light_dir), &n)).dot(dir));
                specular_light_intensity +=
                    specular.powf(material.specular_exponent) * light.intensity;
            }
            let diffuse_color = material.diffuse_color * diffuse_light_intensity;
            let specular_color = Vec3f::new(1., 1., 1.) * specular_light_intensity;

            diffuse_color * material.albedo[0]
                + specular_color * material.albedo[1]
                + reflect_color * material.albedo[2]
                + refract_color * material.albedo[3]
        } else {
            self.scene.background_color(dir)
        }
    }

    pub fn get_image(&self) -> Image {
        let width = 1024;
        let height = 768;

        let fwidth = width as f32;
        let fheight = height as f32;
        let fov = f32::consts::PI / 3.;

        let dir_z = -fheight / (2. * (fov / 2.).tan());

        let buffer: Vec<_> = (0..height)
            .into_par_iter()
            .map(|j| {
                let dir_y = -(j as f32 + 0.5) + fheight / 2.;
                (0..width).into_par_iter().map(move |i| {
                    let dir_x = (i as f32 + 0.5) - fwidth / 2.;
                    let dir = Vec3f::new(dir_x, dir_y, dir_z).normalize();
                    self.cast_ray(&Vec3f::new(0.0, 0., 0.), &dir, 0)
                })
            })
            .flatten()
            .collect();

        Image {
            width,
            height,
            buffer,
        }
    }
}
