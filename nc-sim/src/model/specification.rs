use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigSpecification {

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OverhaulBlock {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_names: Vec<String>,
}