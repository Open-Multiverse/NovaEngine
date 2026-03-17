//! 材质组件封装
//!
//! 提供简化的材质创建接口

use bevy::prelude::*;

/// 材质构建器
///
/// 提供流式 API 创建 PBR 材质
pub struct MaterialBuilder {
    base_color: Color,
    emissive: LinearRgba,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    alpha_mode: AlphaMode,
    double_sided: bool,
    unlit: bool,
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            emissive: LinearRgba::BLACK,
            perceptual_roughness: 0.5,
            metallic: 0.0,
            reflectance: 0.5,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
            unlit: false,
        }
    }
}

impl MaterialBuilder {
    /// 创建新的材质构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置基础颜色
    pub fn color(mut self, color: Color) -> Self {
        self.base_color = color;
        self
    }

    /// 设置 RGB 颜色
    pub fn rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.base_color = Color::srgb(r, g, b);
        self
    }

    /// 设置自发光颜色
    pub fn emissive(mut self, color: LinearRgba) -> Self {
        self.emissive = color;
        self
    }

    /// 设置粗糙度 (0.0 = 光滑, 1.0 = 粗糙)
    pub fn roughness(mut self, roughness: f32) -> Self {
        self.perceptual_roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// 设置金属度 (0.0 = 非金属, 1.0 = 金属)
    pub fn metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic.clamp(0.0, 1.0);
        self
    }

    /// 设置反射率
    pub fn reflectance(mut self, reflectance: f32) -> Self {
        self.reflectance = reflectance.clamp(0.0, 1.0);
        self
    }

    /// 设置为透明模式
    pub fn transparent(mut self) -> Self {
        self.alpha_mode = AlphaMode::Blend;
        self
    }

    /// 设置为遮罩模式
    pub fn masked(mut self, cutoff: f32) -> Self {
        self.alpha_mode = AlphaMode::Mask(cutoff);
        self
    }

    /// 设置双面渲染
    pub fn double_sided(mut self) -> Self {
        self.double_sided = true;
        self
    }

    /// 设置为无光照模式
    pub fn unlit(mut self) -> Self {
        self.unlit = true;
        self
    }

    /// 构建 StandardMaterial
    pub fn build(self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            emissive: self.emissive,
            perceptual_roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            alpha_mode: self.alpha_mode,
            double_sided: self.double_sided,
            unlit: self.unlit,
            ..default()
        }
    }

    /// 构建并添加到资源
    pub fn build_handle(
        self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        materials.add(self.build())
    }
}

/// 预定义材质
pub struct PredefinedMaterials;

impl PredefinedMaterials {
    /// 红色材质
    pub fn red() -> MaterialBuilder {
        MaterialBuilder::new().rgb(0.8, 0.2, 0.2)
    }

    /// 绿色材质
    pub fn green() -> MaterialBuilder {
        MaterialBuilder::new().rgb(0.2, 0.8, 0.2)
    }

    /// 蓝色材质
    pub fn blue() -> MaterialBuilder {
        MaterialBuilder::new().rgb(0.2, 0.2, 0.8)
    }

    /// 白色材质
    pub fn white() -> MaterialBuilder {
        MaterialBuilder::new().rgb(1.0, 1.0, 1.0)
    }

    /// 黑色材质
    pub fn black() -> MaterialBuilder {
        MaterialBuilder::new().rgb(0.1, 0.1, 0.1)
    }

    /// 金属材质
    pub fn metal(color: Color) -> MaterialBuilder {
        MaterialBuilder::new()
            .color(color)
            .metallic(1.0)
            .roughness(0.3)
    }

    /// 玻璃材质
    pub fn glass() -> MaterialBuilder {
        MaterialBuilder::new()
            .rgb(0.9, 0.95, 1.0)
            .transparent()
            .roughness(0.0)
            .metallic(0.0)
    }

    /// 地面材质
    pub fn ground() -> MaterialBuilder {
        MaterialBuilder::new().rgb(0.3, 0.5, 0.3).roughness(0.9)
    }
}
