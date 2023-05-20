#[path = "bank.rs"]
mod bank;
#[path = "company.rs"]
mod company;
#[path = "ipo_result.rs"]
mod ipo_result;
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

use self::company::CompanyApplication;
use self::ipo_result::IPOResult;

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

// pub async fn init() {
//     let token = get_auth_header().await.unwrap();
//     // let banks: Vec<Bank> = get_banks(headers.clone()).await.unwrap();
//     // let results: Vec<CompanyApplication> = get_application_report(headers.clone()).await.unwrap();
// }

async fn get_auth_header() -> Result<HashMap<String, String>, Error> {
    let url = BASE_URL.to_string() + "auth/";
    let body = json!({
        "clientId":"160",
        "username":"00015818",
        "password":"@143Meroshare",
    });

    let result = make_request(&url, Method::POST, Some(body), None).await;
    match result {
        Ok(value) => {
            let token = value
                .headers()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let mut headers = HashMap::new();
            headers.insert(String::from("Authorization"), token.as_str().to_string());
            Ok(headers)
        }
        Err(error) => Err(error),
    }
}

async fn get_banks(headers: HashMap<String, String>) -> Result<Vec<Bank>, Error> {
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
struct ApiResponseCurrentIssue {
    object: Vec<Company>,
}
pub async fn get_current_issue() -> Result<Vec<Company>, Error> {
    let headers = get_auth_header().await.unwrap();
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
            let response: ApiResponseCurrentIssue = value.json().await.unwrap();
            Ok(response.object)
        }
        Err(error) => Err(error),
    }
}
#[derive(Debug, Deserialize)]

struct ApiResponseApplicationReport {
    object: Vec<CompanyApplication>,
}

async fn get_application_report(
    headers: HashMap<String, String>,
) -> Result<Vec<CompanyApplication>, Error> {
    let url = BASE_URL.to_string() + "applicantForm/active/search/";
    let body = json!({
        "filterFieldParams": [
            {
                "key": "companyShare.companyIssue.companyISIN.script",
                "alias": "Scrip"
            },
            {
                "key": "companyShare.companyIssue.companyISIN.company.name",
                "alias": "Company Name"
            }
        ],
        "page": 1,
        "size": 200,
        "searchRoleViewConstants": "VIEW_APPLICANT_FORM_COMPLETE",
        "filterDateParams": [
            {
                "key": "appliedDate",
                "condition": "",
                "alias": "",
                "value": ""
            },
            {
                "key": "appliedDate",
                "condition": "",
                "alias": "",
                "value": ""
            }
        ]
    });
    let result = make_request(&url, Method::POST, Some(body), Some(headers)).await;
    match result {
        Ok(value) => {
            let response: ApiResponseApplicationReport = value.json().await?;
            Ok(response.object)
        }
        Err(error) => Err(error),
    }
}

async fn get_company_result(
    headers: HashMap<String, String>,
    application: CompanyApplication,
) -> Result<IPOResult, Error> {
    let url = BASE_URL.to_string()
        + "applicantForm/report/detail/"
        + (application.id).to_string().as_str();
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let result: IPOResult = value.json().await?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}
