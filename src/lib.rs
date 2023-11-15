#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Expression3 {
  pub fade_in_time: Option<f64>,
  pub fade_out_time: Option<f64>,
  pub parameters: Vec<Parameter>,
  #[serde(rename = "Type")]
  pub type_: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[remain::sorted]
pub enum Type {
  #[serde(rename = "Live2D Expression")]
  Live2D,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Parameter {
  pub blend: Option<Blend>,
  pub id: String,
  pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[remain::sorted]
pub enum Blend {
  Add,
  Multiply,
  Overwrite,
}
