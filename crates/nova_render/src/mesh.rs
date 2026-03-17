//! 网格组件封装
//!
//! 提供简化的网格创建接口

use bevy::prelude::*;

/// 预定义网格形状
#[derive(Debug, Clone)]
pub enum MeshShape {
    /// 立方体
    Cube { size: f32 },
    /// 长方体
    Box { width: f32, height: f32, depth: f32 },
    /// 球体
    Sphere { radius: f32, subdivisions: usize },
    /// 平面
    Plane { size: f32 },
    /// 圆柱体
    Cylinder { radius: f32, height: f32 },
    /// 胶囊体
    Capsule { radius: f32, depth: f32 },
    /// 圆环
    Torus { radius: f32, ring_radius: f32 },
}

impl MeshShape {
    /// 创建立方体
    pub fn cube(size: f32) -> Self {
        Self::Cube { size }
    }

    /// 创建长方体
    pub fn box_shape(width: f32, height: f32, depth: f32) -> Self {
        Self::Box {
            width,
            height,
            depth,
        }
    }

    /// 创建球体
    pub fn sphere(radius: f32) -> Self {
        Self::Sphere {
            radius,
            subdivisions: 32,
        }
    }

    /// 创建平面
    pub fn plane(size: f32) -> Self {
        Self::Plane { size }
    }

    /// 创建圆柱体
    pub fn cylinder(radius: f32, height: f32) -> Self {
        Self::Cylinder { radius, height }
    }

    /// 创建胶囊体
    pub fn capsule(radius: f32, depth: f32) -> Self {
        Self::Capsule { radius, depth }
    }

    /// 创建圆环
    pub fn torus(radius: f32, ring_radius: f32) -> Self {
        Self::Torus {
            radius,
            ring_radius,
        }
    }

    /// 转换为 Bevy Mesh
    pub fn to_mesh(&self) -> Mesh {
        match self {
            MeshShape::Cube { size } => Cuboid::new(*size, *size, *size).into(),
            MeshShape::Box {
                width,
                height,
                depth,
            } => Cuboid::new(*width, *height, *depth).into(),
            MeshShape::Sphere {
                radius,
                subdivisions,
            } => Sphere::new(*radius)
                .mesh()
                .ico(*subdivisions as u32)
                .unwrap(),
            MeshShape::Plane { size } => Plane3d::default().mesh().size(*size, *size).into(),
            MeshShape::Cylinder { radius, height } => Cylinder::new(*radius, *height).into(),
            MeshShape::Capsule { radius, depth } => Capsule3d::new(*radius, *depth).into(),
            MeshShape::Torus {
                radius,
                ring_radius,
            } => Torus::new(*ring_radius, *radius).into(),
        }
    }
}

/// Nova 网格构建器
pub struct NovaMeshBuilder {
    shape: MeshShape,
}

impl NovaMeshBuilder {
    /// 创建新的网格构建器
    pub fn new(shape: MeshShape) -> Self {
        Self { shape }
    }

    /// 构建网格并添加到资源
    pub fn build(self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        meshes.add(self.shape.to_mesh())
    }
}

/// 创建基础网格的辅助函数
pub fn create_cube_mesh(meshes: &mut Assets<Mesh>, size: f32) -> Handle<Mesh> {
    NovaMeshBuilder::new(MeshShape::cube(size)).build(meshes)
}

pub fn create_sphere_mesh(meshes: &mut Assets<Mesh>, radius: f32) -> Handle<Mesh> {
    NovaMeshBuilder::new(MeshShape::sphere(radius)).build(meshes)
}

pub fn create_plane_mesh(meshes: &mut Assets<Mesh>, size: f32) -> Handle<Mesh> {
    NovaMeshBuilder::new(MeshShape::plane(size)).build(meshes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_shape_cube() {
        let shape = MeshShape::cube(2.0);
        match shape {
            MeshShape::Cube { size } => assert_eq!(size, 2.0),
            _ => panic!("Expected Cube"),
        }
    }

    #[test]
    fn test_mesh_shape_box() {
        let shape = MeshShape::box_shape(1.0, 2.0, 3.0);
        match shape {
            MeshShape::Box {
                width,
                height,
                depth,
            } => {
                assert_eq!(width, 1.0);
                assert_eq!(height, 2.0);
                assert_eq!(depth, 3.0);
            }
            _ => panic!("Expected Box"),
        }
    }

    #[test]
    fn test_mesh_shape_sphere() {
        let shape = MeshShape::sphere(1.5);
        match shape {
            MeshShape::Sphere {
                radius,
                subdivisions,
            } => {
                assert_eq!(radius, 1.5);
                assert_eq!(subdivisions, 32);
            }
            _ => panic!("Expected Sphere"),
        }
    }

    #[test]
    fn test_mesh_shape_plane() {
        let shape = MeshShape::plane(10.0);
        match shape {
            MeshShape::Plane { size } => assert_eq!(size, 10.0),
            _ => panic!("Expected Plane"),
        }
    }

    #[test]
    fn test_mesh_shape_to_mesh() {
        // 测试网格生成不会 panic
        let shapes = vec![
            MeshShape::cube(1.0),
            MeshShape::box_shape(1.0, 2.0, 3.0),
            MeshShape::sphere(1.0),
            MeshShape::plane(5.0),
            MeshShape::cylinder(0.5, 2.0),
            MeshShape::capsule(0.5, 1.0),
            MeshShape::torus(1.0, 0.3),
        ];

        for shape in shapes {
            let _mesh = shape.to_mesh();
        }
    }

    #[test]
    fn test_nova_mesh_builder() {
        let builder = NovaMeshBuilder::new(MeshShape::cube(1.0));
        assert!(matches!(builder.shape, MeshShape::Cube { .. }));
    }
}
