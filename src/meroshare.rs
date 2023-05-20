#[path = "bank.rs"]
mod bank;
#[path = "company.rs"]
mod company;
#[path = "request.rs"]
mod request;

use bank::Bank;
use company::Company;
use request::make_request;
use reqwest::Error;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
// {
//     dp: "16700",
//     username: "15818",
//     password: "@143Meroshare",
//     crn: "U0-R01799852",
//     pin: "2056",
//     name: "Ashok",
//     asbaBankIndex: 2,
//   },

const BASE_URL: &str = "https://backend.cdsc.com.np/api/meroShare/";

pub async fn init() {
    let token = get_auth_token().await.unwrap();
    let mut headers = HashMap::new();
    headers.insert("Authorization", token.as_str());
    let banks: Vec<Bank> = get_banks(headers.clone()).await.unwrap();
    let companies: Vec<Company> = get_current_issue(headers.clone()).await.unwrap();
}

async fn get_auth_token() -> Result<String, Error> {
    let url = BASE_URL.to_string() + "auth/";
    let body = json!({
        "clientId":"160",
        "username":"00015818",
        "password":"@143Meroshare",
    });

    let result = make_request(&url, Method::POST, Some(body), None).await;
    match result {
        Ok(value) => Ok(value
            .headers()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()),
        Err(error) => Err(error),
    }
}
async fn get_banks(headers: HashMap<&str, &str>) -> Result<Vec<Bank>, Error> {
    let url = BASE_URL.to_string() + "bank/";
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let banks: Vec<Bank> = value.json().await?;
            Ok(banks)
        }
        Err(error) => Err(error),
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    object: Vec<Company>,
}

async fn get_current_issue(headers: HashMap<&str, &str>) -> Result<Vec<Company>, Error> {
    let url = BASE_URL.to_string() + "companyShare/currentIssue/";
    let body = json!({
        "filterFieldParams": [
            {"key": "companyIssue.companyISIN.script", "alias":"Scrip"},
            {"key": "companyIssue.companyISIN.company.name", "alias": "Company Name"},
            {"key": "companyIssue.assignedToClient.name", "value":"", "alias": "Issue Manager"}
        ],
        "page":1,
        "size":200,
        "searchRoleViewConstants":"VIEW_OPEN_SHARE",
        "filterDateParams":[
            {"key": "minIssueOpenDate", "condition": "", "alias": "", "value": ""},
            {"key": "maxIssueCloseDate", "condition": "", "alias":"", "value": ""}
        ]
    });
    let result = make_request(&url, Method::POST, Some(body), Some(headers)).await;
    match result {
        Ok(value) => {
            let response: ApiResponse = value.json().await?;
            Ok(response.object)
        }
        Err(error) => Err(error),
    }
}
