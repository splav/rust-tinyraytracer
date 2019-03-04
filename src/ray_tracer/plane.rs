use super::material::Material;
use super::Vec3f;
use std::f32;

pub struct Plane {
    pub material: Material,
}

impl Plane {
    pub fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> (f32, (Vec3f, Vec3f, Material)) {
        let mut dist = f32::MAX;
        let mut hit_point = Vec3f::zeros();
        let n = Vec3f::new(0., 1., 0.);
        let mut material = self.material.clone();

        if dir.y.abs() > 1e-3 {
            let d = -(orig.y + 4.) / dir.y; // the checkerboard plane has equation y = -4
            let pt = orig + dir * d;
            if d > 0. && pt.x.abs() < 10. && pt.z < -10. && pt.z > -30. && d < dist {
                dist = d;
                hit_point = pt;
                let column = (0.5 * hit_point.x) as i64;
                let row = (0.5 * hit_point.z) as i64;
                let is_white = ((column + row) & 1) > 0;
                material.diffuse_color = if is_white {
                    Vec3f::new(0.3, 0.3, 0.3)
                } else {
                    Vec3f::new(0.3, 0.2, 0.1)
                };
            }
        }
        (dist, (hit_point, n, material))
    }
}
