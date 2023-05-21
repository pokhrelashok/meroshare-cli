use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IPOResult {
    // #[serde(rename = "appliedKitta")]
    // pub applied_kitta: i32,
    #[serde(rename = "statusName")]
    pub status: String,
}
