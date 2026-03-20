//! 角色配置加载 - 支持从 JSON 定义角色

use serde::{Deserialize, Serialize};

use crate::attributes::Attributes;
use crate::character::CharacterType;

/// 属性定义（JSON 中的数值）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributesDef {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub vision_range: f32,
}

impl AttributesDef {
    pub fn to_attributes(&self) -> Attributes {
        use crate::attributes::Health;
        Attributes {
            health: Health::new(self.health),
            attack: self.attack,
            defense: self.defense,
            move_speed: self.move_speed,
            attack_range: self.attack_range,
            attack_speed: self.attack_speed,
            vision_range: self.vision_range,
        }
    }
}

/// 模型定义
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelDef {
    Primitive {
        shape: PrimitiveShapeDef,
        color: [f32; 4],
    },
    Gltf {
        path: String,
    },
}

/// 原始形状定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrimitiveShapeDef {
    Capsule { radius: f32, height: f32 },
    Cube { size: f32 },
    Sphere { radius: f32 },
}

/// 性格定义（可选）
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PersonalityDef {
    #[serde(default = "default_0_5")]
    pub aggression: f32,
    #[serde(default = "default_0_5")]
    pub courage: f32,
    #[serde(default = "default_0_5")]
    pub discipline: f32,
}

fn default_0_5() -> f32 {
    0.5
}

/// 角色定义
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterDef {
    pub id: String,
    pub name: String,
    pub character_type: String,
    pub attributes: AttributesDef,
    pub model: ModelDef,
    #[serde(default)]
    pub personality: PersonalityDef,
}

impl CharacterDef {
    pub fn character_type(&self) -> CharacterType {
        match self.character_type.as_str() {
            "Infantry" => CharacterType::Infantry,
            "Archer" => CharacterType::Archer,
            "Mage" => CharacterType::Mage,
            "Knight" => CharacterType::Knight,
            _ => CharacterType::Infantry,
        }
    }
}

/// 角色配置文件（包含多个角色定义）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub characters: Vec<CharacterDef>,
}

impl CharacterConfig {
    /// 从 JSON 字符串加载
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 按 id 查找角色定义
    pub fn find(&self, id: &str) -> Option<&CharacterDef> {
        self.characters.iter().find(|c| c.id == id)
    }
}
