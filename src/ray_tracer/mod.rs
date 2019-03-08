use nalgebra::{Vector3, Vector4};

pub type Vec3f = Vector3<f32>;
pub type Vec3u = Vector3<usize>;
pub type Vec4f = Vector4<f32>;

mod raster;
mod light;
mod material;
mod model;
mod plane;
mod render;
mod scene;
mod sphere;

pub use raster::Image;
pub use light::Light;
pub use material::Material;
pub use model::Geometry;
pub use model::Model;
pub use plane::Plane;
pub use render::Render;
pub use scene::Scene;
pub use sphere::Sphere;
