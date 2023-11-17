#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[remain::sorted]
pub enum Type {
  #[serde(rename = "Live2D Expression")]
  Live2D,
}
