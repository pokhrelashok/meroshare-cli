use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Company {
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "companyShareId")]
    pub company_share_id: i32,
    #[serde(rename = "issueCloseDate")]
    pub issue_close_date: String,
    #[serde(rename = "issueOpenDate")]
    issue_open_date: String,
    #[serde(rename = "reservationTypeName")]
    reservation_type_name: String,
    #[serde(rename = "scrip")]
    script: String,
    #[serde(rename = "shareGroupName")]
    share_group_name: String,
    #[serde(rename = "shareTypeName")]
    pub share_type_name: String,
    #[serde(rename = "statusName")]
    status_name: String,
    #[serde(rename = "subGroup")]
    sub_group: String,
}
#[derive(Debug, Deserialize)]

pub struct CompanyApplication {
    #[serde(rename = "applicantFormId")]
    pub id: u64,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "companyShareId")]
    pub company_share_id: i32,
    #[serde(rename = "scrip")]
    pub script: String,
    #[serde(rename = "shareGroupName")]
    pub share_group_name: String,
    #[serde(rename = "shareTypeName")]
    pub share_type_name: String,
    #[serde(rename = "statusName")]
    pub status_name: String,
    #[serde(rename = "subGroup")]
    pub sub_group: String,
}
