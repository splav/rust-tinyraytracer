use super::material::Material;
use super::Vec3f;

#[derive(Debug, Clone)]
pub struct Sphere<'a> {
    pub center: Vec3f,
    pub radius: f32,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let l = self.center - orig;
        let tca = dir.dot(&l);
        let d2 = l.dot(&l) - tca * tca;
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }

        let thc = (r2 - d2).sqrt();
        let mut t = tca - thc;
        if t < 0. {
            t = tca + thc;
        }
        if t < 0.0 {
            None
        } else {
            Some(t)
        }
    }
}
