use prettytable::{Attr, Cell, Row, Table};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub dp: String,
    pub username: String,
    pub password: String,
    pub crn: String,
    pub pin: String,
    pub name: String,
    #[serde(rename = "asbaBankIndex")]
    pub bank_index: usize,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDetails {
    pub address: String,
    pub boid: String,
    #[serde(rename = "clientCode")]
    pub client_code: String,
    pub contact: String,
    #[serde(rename = "createdApproveDate")]
    pub created_approve_date: String,
    #[serde(rename = "createdApproveDateStr")]
    pub created_approve_date_str: String,
    #[serde(rename = "customerTypeCode")]
    pub customer_type_code: String,
    pub demat: String,
    #[serde(rename = "dematExpiryDate")]
    pub demat_expiry_date: String,
    pub email: String,
    #[serde(rename = "expiredDate")]
    pub expired_date: String,
    #[serde(rename = "expiredDateStr")]
    pub expired_date_str: String,
    pub gender: String,
    pub id: u32,
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(rename = "meroShareEmail")]
    pub mero_share_email: String,
    pub name: String,
    #[serde(rename = "passwordChangeDate")]
    pub password_change_date: String,
    #[serde(rename = "passwordChangedDateStr")]
    pub password_changed_date_str: String,
    #[serde(rename = "passwordExpiryDate")]
    pub password_expiry_date: String,
    #[serde(rename = "passwordExpiryDateStr")]
    pub password_expiry_date_str: String,
    #[serde(rename = "profileName")]
    pub profile_name: String,
    #[serde(rename = "renderDashboard")]
    pub render_dashboard: bool,
    #[serde(rename = "renewedDate")]
    pub renewed_date: String,
    #[serde(rename = "renewedDateStr")]
    pub renewed_date_str: String,
    pub username: String,
}

pub fn print_users(users: &Vec<User>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("S.N.").with_style(Attr::Bold),
        Cell::new("Name").with_style(Attr::Bold),
    ]));
    for (i, _) in users.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new((i + 1).to_string().as_str()),
            Cell::new(users.get(i).unwrap().name.as_str()),
        ]));
    }
    table.printstd();
}
