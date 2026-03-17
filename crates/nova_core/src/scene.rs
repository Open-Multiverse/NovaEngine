//! 场景系统
//!
//! 提供场景定义、序列化和加载功能

use bevy::prelude::*;
use std::collections::HashMap;

/// 场景实体定义
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SceneEntity {
    /// 实体名称
    pub name: String,
    /// 位置
    #[serde(default)]
    pub position: [f32; 3],
    /// 旋转（欧拉角，度数）
    #[serde(default)]
    pub rotation: [f32; 3],
    /// 缩放
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
    /// 组件类型
    #[serde(default)]
    pub components: Vec<SceneComponent>,
    /// 子实体
    #[serde(default)]
    pub children: Vec<SceneEntity>,
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

impl Default for SceneEntity {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            components: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl SceneEntity {
    /// 创建新的场景实体
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    /// 设置位置
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = [x, y, z];
        self
    }

    /// 设置旋转
    pub fn with_rotation(mut self, x: f32, y: f32, z: f32) -> Self {
        self.rotation = [x, y, z];
        self
    }

    /// 设置缩放
    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.scale = [x, y, z];
        self
    }

    /// 添加组件
    pub fn with_component(mut self, component: SceneComponent) -> Self {
        self.components.push(component);
        self
    }

    /// 添加子实体
    pub fn with_child(mut self, child: SceneEntity) -> Self {
        self.children.push(child);
        self
    }

    /// 获取 Transform
    pub fn to_transform(&self) -> Transform {
        Transform {
            translation: Vec3::from_array(self.position),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                self.rotation[0].to_radians(),
                self.rotation[1].to_radians(),
                self.rotation[2].to_radians(),
            ),
            scale: Vec3::from_array(self.scale),
        }
    }
}

/// 场景组件定义
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SceneComponent {
    /// 相机
    Camera {
        #[serde(default = "default_fov")]
        fov: f32,
    },
    /// 方向光
    DirectionalLight {
        #[serde(default = "default_color")]
        color: [f32; 3],
        #[serde(default = "default_intensity")]
        intensity: f32,
    },
    /// 点光源
    PointLight {
        #[serde(default = "default_color")]
        color: [f32; 3],
        #[serde(default = "default_intensity")]
        intensity: f32,
        #[serde(default = "default_range")]
        range: f32,
    },
    /// 网格
    Mesh {
        shape: MeshShapeType,
    },
    /// 刚体
    RigidBody {
        #[serde(default)]
        body_type: RigidBodyType,
    },
    /// 碰撞器
    Collider {
        shape: ColliderShapeType,
    },
    /// 自定义组件（键值对）
    Custom {
        name: String,
        #[serde(default)]
        properties: HashMap<String, serde_json::Value>,
    },
}

fn default_fov() -> f32 {
    60.0
}
fn default_color() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}
fn default_intensity() -> f32 {
    1.0
}
fn default_range() -> f32 {
    10.0
}

/// 网格形状类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MeshShapeType {
    Cube { size: f32 },
    Sphere { radius: f32 },
    Plane { size: f32 },
    Cylinder { radius: f32, height: f32 },
    Capsule { radius: f32, height: f32 },
}

/// 刚体类型
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum RigidBodyType {
    #[default]
    Dynamic,
    Static,
    Kinematic,
}

/// 碰撞器形状类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ColliderShapeType {
    Box { half_extents: [f32; 3] },
    Sphere { radius: f32 },
    Capsule { radius: f32, height: f32 },
}

/// 场景定义
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SceneDefinition {
    /// 场景名称
    pub name: String,
    /// 场景描述
    #[serde(default)]
    pub description: String,
    /// 版本
    #[serde(default = "default_version")]
    pub version: String,
    /// 根实体列表
    #[serde(default)]
    pub entities: Vec<SceneEntity>,
    /// 场景设置
    #[serde(default)]
    pub settings: SceneSettings,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// 场景设置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SceneSettings {
    /// 环境光颜色
    #[serde(default = "default_ambient")]
    pub ambient_color: [f32; 3],
    /// 环境光强度
    #[serde(default = "default_ambient_intensity")]
    pub ambient_intensity: f32,
    /// 背景颜色
    #[serde(default = "default_background")]
    pub background_color: [f32; 3],
    /// 重力
    #[serde(default = "default_gravity")]
    pub gravity: [f32; 3],
}

fn default_ambient() -> [f32; 3] {
    [0.1, 0.1, 0.1]
}
fn default_ambient_intensity() -> f32 {
    0.2
}
fn default_background() -> [f32; 3] {
    [0.1, 0.1, 0.15]
}
fn default_gravity() -> [f32; 3] {
    [0.0, -9.81, 0.0]
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            ambient_color: default_ambient(),
            ambient_intensity: default_ambient_intensity(),
            background_color: default_background(),
            gravity: default_gravity(),
        }
    }
}

impl SceneDefinition {
    /// 创建新场景
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    /// 添加实体
    pub fn with_entity(mut self, entity: SceneEntity) -> Self {
        self.entities.push(entity);
        self
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// 从 JSON 字符串加载
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 序列化为 JSON 字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从 JSON 字符串加载（紧凑格式）
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// 当前场景资源
#[derive(Resource, Default)]
pub struct CurrentScene {
    /// 场景定义
    pub definition: Option<SceneDefinition>,
    /// 场景名称
    pub name: String,
    /// 是否已加载
    pub loaded: bool,
}

/// 场景加载事件
#[derive(Event, Debug)]
pub struct LoadSceneEvent {
    /// 场景 JSON 内容
    pub json: String,
}

/// 场景加载完成事件
#[derive(Event, Debug)]
pub struct SceneLoadedEvent {
    /// 场景名称
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_entity_default() {
        let entity = SceneEntity::default();
        assert_eq!(entity.position, [0.0, 0.0, 0.0]);
        assert_eq!(entity.rotation, [0.0, 0.0, 0.0]);
        assert_eq!(entity.scale, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_scene_entity_builder() {
        let entity = SceneEntity::new("player")
            .with_position(1.0, 2.0, 3.0)
            .with_rotation(0.0, 90.0, 0.0)
            .with_scale(2.0, 2.0, 2.0);

        assert_eq!(entity.name, "player");
        assert_eq!(entity.position, [1.0, 2.0, 3.0]);
        assert_eq!(entity.rotation, [0.0, 90.0, 0.0]);
        assert_eq!(entity.scale, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_scene_entity_to_transform() {
        let entity = SceneEntity::new("test")
            .with_position(1.0, 2.0, 3.0)
            .with_scale(2.0, 2.0, 2.0);

        let transform = entity.to_transform();
        assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.scale, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_scene_definition_builder() {
        let scene = SceneDefinition::new("TestScene")
            .with_description("A test scene")
            .with_entity(SceneEntity::new("camera"))
            .with_entity(SceneEntity::new("light"));

        assert_eq!(scene.name, "TestScene");
        assert_eq!(scene.description, "A test scene");
        assert_eq!(scene.entities.len(), 2);
    }

    #[test]
    fn test_scene_json_roundtrip() {
        let scene = SceneDefinition::new("TestScene")
            .with_entity(
                SceneEntity::new("cube")
                    .with_position(0.0, 1.0, 0.0)
                    .with_component(SceneComponent::Mesh {
                        shape: MeshShapeType::Cube { size: 1.0 },
                    }),
            );

        let json = scene.to_json().unwrap();
        let loaded = SceneDefinition::from_json(&json).unwrap();

        assert_eq!(loaded.name, "TestScene");
        assert_eq!(loaded.entities.len(), 1);
        assert_eq!(loaded.entities[0].name, "cube");
    }

    #[test]
    fn test_scene_settings_default() {
        let settings = SceneSettings::default();
        assert_eq!(settings.gravity, [0.0, -9.81, 0.0]);
        assert!((settings.ambient_intensity - 0.2).abs() < 0.001);
    }
}
