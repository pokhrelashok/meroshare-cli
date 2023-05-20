use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Company {
    #[serde(rename = "companyName")]
    company_name: String,
    #[serde(rename = "companyShareId")]
    company_share_id: i32,
    #[serde(rename = "issueCloseDate")]
    issue_close_date: String,
    #[serde(rename = "issueOpenDate")]
    issue_open_date: String,
    #[serde(rename = "reservationTypeName")]
    reservation_type_name: String,
    #[serde(rename = "scrip")]
    scrip: String,
    #[serde(rename = "shareGroupName")]
    share_group_name: String,
    #[serde(rename = "shareTypeName")]
    share_type_name: String,
    #[serde(rename = "statusName")]
    status_name: String,
    #[serde(rename = "subGroup")]
    sub_group: String,
}
