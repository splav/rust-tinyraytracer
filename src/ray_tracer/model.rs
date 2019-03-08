use std::f32;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use nalgebra::{inf, sup};

use super::{Material, Vec3f, Vec3u};

#[derive(Clone, Debug)]
pub struct Geometry {
    verts: Vec<Vec3f>,
    faces: Vec<Vec3u>,
    bbox: AABBox,
}

#[derive(Clone, Debug)]
pub struct AABBox {
    bounds: [Vec3f; 2],
}

#[derive(Clone, Debug)]
pub struct Model<'a> {
    pub geometry: Geometry,
    pub material: &'a Material,
}

impl<'a> Model<'a> {
    pub fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> (f32, (Vec3f, Vec3f, Material)) {
        let mut dist = f32::MAX;
        let mut hit_point = Vec3f::zeros();
        let mut n = Vec3f::zeros();
        let mut material = Material::default();

        if self.geometry.bbox.intersects(orig, dir) {
            for triangle in &self.geometry.faces {
                let vertexes = (
                    self.geometry.point(triangle[0]),
                    self.geometry.point(triangle[1]),
                    self.geometry.point(triangle[2]),
                );
                if let Some((dist_i, point_i, n_i)) = ray_triangle_intersect(vertexes, orig, dir) {
                    if dist_i >= dist {
                        continue;
                    }
                    dist = dist_i;
                    n = n_i;
                    hit_point = point_i;
                    material = self.material.clone();
                }
            }
        }
        (dist, (hit_point, n, material))
    }
}

impl Geometry {
    pub fn load(file_name: &str) -> Self {
        let file = File::open(file_name).expect("failed to open model");
        let buf = BufReader::new(file);

        let mut verts = Vec::<Vec3f>::new();
        let mut faces = Vec::<Vec3u>::new();

        for l in buf.lines() {
            let line = l.expect("failed to read line");

            let s: Vec<_> = line.split(' ').collect();
            if let Some((key, vals)) = s.split_first() {
                match *key {
                    "v" => {
                        let num_iter = vals
                            .iter()
                            .map(|val| val.parse::<f32>())
                            .filter_map(Result::ok);
                        verts.push(Vec3f::from_iterator(num_iter));
                    }
                    "f" => {
                        faces.push(
                            Vec3u::from_iterator(
                                vals.iter().map(|val| val.parse()).filter_map(Result::ok),
                            ) - Vec3u::new(1, 1, 1),
                        );
                        // break;
                    }
                    _ => {}
                }
            }
        }
        eprintln!("# v#{}, f#{}", verts.len(), faces.len());
        let bbox = AABBox::from_verts(&verts);

        Self { verts, faces, bbox }
    }

    pub fn point(&self, i: usize) -> Vec3f {
        self.verts[i]
    }
}

// Moller and Trumbore
fn ray_triangle_intersect(
    (vert0, vert1, vert2): (Vec3f, Vec3f, Vec3f),
    orig: &Vec3f,
    dir: &Vec3f,
) -> Option<(f32, Vec3f, Vec3f)> {
    let edge1 = vert1 - vert0;
    let edge2 = vert2 - vert0;
    let h = dir.cross(&edge2);
    let det = edge1.dot(&h);
    if det.abs() < 1e-5 {
        return None;
    }
    let l_det = 1. / det;

    let svec = orig - vert0;
    let u = l_det * svec.dot(&h);
    if u < 0. || u > 1. {
        return None;
    }

    let qvec = svec.cross(&edge1);
    let v = l_det * dir.dot(&qvec);
    if v < 0. || u + v > 1. {
        return None;
    }

    let tnear = l_det * edge2.dot(&qvec);
    if tnear > 1e-5 {
        let n = edge1.cross(&edge2).normalize();
        Some((tnear, orig + dir * tnear, n))
    } else {
        None
    }
}

impl AABBox {
    fn from_verts(verts: &[Vec3f]) -> Self {
        let minmax = dbg!(verts.iter().fold([verts[0], verts[0]], |[min, max], x| [
            inf(&min, x),
            sup(&max, x)
        ]));

        Self { bounds: minmax }
    }

    fn intersects(&self, orig: &Vec3f, dir: &Vec3f) -> bool {
        let invdir = Vec3f::new(1. / dir.x, 1. / dir.y, 1. / dir.z);
        let signs = Vec3u::new(
            (invdir[0] < 0.) as usize,
            (invdir[1] < 0.) as usize,
            (invdir[2] < 0.) as usize,
        );

        let mut tmin = (self.bounds[signs.x].x - orig.x) * invdir.x;
        let mut tmax = (self.bounds[1 - signs.x].x - orig.x) * invdir.x;
        let tymin = (self.bounds[signs.y].y - orig.y) * invdir.y;
        let tymax = (self.bounds[1 - signs.y].y - orig.y) * invdir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[signs.z].z - orig.z) * invdir.z;
        let tzmax = (self.bounds[1 - signs.z].z - orig.z) * invdir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }
        true
    }
}
