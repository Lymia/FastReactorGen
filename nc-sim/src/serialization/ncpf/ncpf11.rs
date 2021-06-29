use serde::*;
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NCPF11 {
    pub addon: bool,
    pub name: String,
    pub version: String,
    pub underhaul_version: String,

    pub overhaul: OverhaulConfiguration,
    // TODO: addons
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct TextureInfo(pub Vec<i32>);
impl Debug for TextureInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() || self.0[0] <= 0 {
            write!(f, "[invalid texture]")
        } else {
            let size = self.0[0] as usize;
            if self.0.len() != 1 + size * size {
                write!(f, "[invalid texture]")
            } else {
                write!(f, "[{0}x{0} texture]", size)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulConfiguration {
    #[serde(rename = "fissionSFR")]
    pub fission_sfr: OverhaulSFRConfiguration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulSFRConfiguration {
    #[serde(default)]
    pub min_size: Option<i32>,
    #[serde(default)]
    pub max_size: Option<i32>,
    #[serde(default)]
    pub neutron_reach: Option<i32>,
    #[serde(default)]
    pub cooling_efficiency_leniency: Option<i32>,
    #[serde(default)]
    pub sparsity_penalty_mult: Option<f32>,
    #[serde(default)]
    pub sparsity_penalty_threshold: Option<f32>,

    pub blocks: Vec<OverhaulSFRBlock>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulSFRBlock {
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub legacy_names: Vec<String>,
    #[serde(default)]
    pub texture: Option<TextureInfo>,

    #[serde(default)]
    pub cluster: bool,
    #[serde(default)]
    pub create_cluster: bool,
    #[serde(default)]
    pub conductor: bool,
    #[serde(default)]
    pub functional: bool,
    #[serde(default, rename = "blockLOS")]
    pub block_los: bool,
    #[serde(default)]
    pub casing: bool,
    #[serde(default)]
    pub casing_edge: bool,
    #[serde(default)]
    pub controller: bool,

    #[serde(flatten)]
    pub stats: OverhaulStats,
    #[serde(default)]
    pub recipes: Vec<OverhaulSFRRecipe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulSFRRecipe {
    pub input: OverhaulRecipeIO,
    pub output: OverhaulRecipeIO,
    #[serde(flatten)]
    pub stats: OverhaulStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulRecipeIO {
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub legacy_names: Vec<String>,
    #[serde(default)]
    pub texture: Option<TextureInfo>,
    #[serde(default)]
    pub rate: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulStats {
    #[serde(default)]
    pub coolant_vent: Option<OverhaulCoolantVent>,
    #[serde(default)]
    pub fuel_cell: Option<StatsCell<OverhaulFuelCell>>,
    #[serde(default)]
    pub reflector: Option<StatsCell<OverhaulReflector>>,
    #[serde(default)]
    pub moderator: Option<StatsCell<OverhaulModerator>>,
    #[serde(default, rename = "heatsink")]
    pub heat_sink: Option<StatsCell<OverhaulHeatSink>>,
    #[serde(default)]
    pub source: Option<StatsCell<OverhaulSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulCoolantVent {
    #[serde(default)]
    pub out_texture: Option<TextureInfo>,
    pub out_display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatsCell<T> {
    #[serde(default)]
    pub has_base_stats: Option<bool>,
    #[serde(flatten)]
    pub stats: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulFuelCell {
    pub efficiency: f32,
    pub heat: i32,
    pub criticality: i32,
    #[serde(default)]
    pub self_priming: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulReflector {
    pub efficiency: f32,
    pub reflectivity: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulModerator {
    pub flux: i32,
    pub efficiency: f32,
    #[serde(default)]
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulHeatSink {
    pub cooling: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverhaulSource {
    pub cooling: f32,
}
