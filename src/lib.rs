mod blend;
mod parameter;
mod r#type;

pub use blend::Blend;
pub use parameter::Parameter;
pub use r#type::Type;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
