use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IPOResult {
    #[serde(rename = "appliedKitta")]
    applied_kitta: i32,
    #[serde(rename = "statusName")]
    status: String,
}
