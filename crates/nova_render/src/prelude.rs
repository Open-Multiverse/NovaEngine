//! Nova Render Prelude

pub use crate::camera::{MainCamera, NovaCamera3d};
pub use crate::camera_controller::{
    FpsCameraController, FpsCameraPlugin, OrbitCameraController, OrbitCameraPlugin,
};
pub use crate::light::{spawn_directional_light, spawn_point_light, AmbientLightConfig};
pub use crate::material::{MaterialBuilder, PredefinedMaterials};
pub use crate::mesh::{
    create_cube_mesh, create_plane_mesh, create_sphere_mesh, MeshShape, NovaMeshBuilder,
};
pub use crate::NovaRenderPlugin;
