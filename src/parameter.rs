use crate::Blend;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Parameter {
  pub blend: Option<Blend>,
  pub id: String,
  pub value: f64,
}
