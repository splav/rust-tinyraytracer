use super::Vec3f;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}
