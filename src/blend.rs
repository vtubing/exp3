#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[remain::sorted]
pub enum Blend {
  Add,
  Multiply,
  Overwrite,
}
