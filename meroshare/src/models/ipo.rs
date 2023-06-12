use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IPOResult {
    // #[serde(rename = "appliedKitta")]
    // pub applied_kitta: i32,
    #[serde(rename = "statusName")]
    pub status: String,
}

#[derive(Debug, Deserialize)]

pub struct IPOAppliedResult {
    pub message: String,
    pub status: String,
}
