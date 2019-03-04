use super::{Vec3f, Vec4f};

#[derive(Debug, Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: Vec4f,
    pub diffuse_color: Vec3f,
    pub specular_exponent: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            refractive_index: 1.0,
            albedo: Vec4f::new(1., 0., 0., 0.),
            diffuse_color: Vec3f::zeros(),
            specular_exponent: 0.,
        }
    }
}
