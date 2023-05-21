use reqwest::Error;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

use crate::bank::Bank;
use crate::company::Company;
use crate::company::CompanyApplication;
use crate::file::get_user_stored_token;
use crate::file::store_user_token;
use crate::ipo_result::IPOResult;
use crate::request::make_request;
use crate::user::get_users;
use crate::user::User;

const BASE_URL: &str = "https://backend.cdsc.com.np/api/meroShare/";

async fn get_auth_header(user: Option<User>) -> Result<HashMap<String, String>, Error> {
    let user_data: User = user.unwrap_or_else(|| get_users().get(0).unwrap().clone());
    let mut token: String = String::new();
    match get_user_stored_token(&user_data.username) {
        Some(t) => {
            token = t;
        }
        None => {
            let body = json!({
                "clientId":user_data.dp,
                "username":user_data.username,
                "password":user_data.password,
            });
            let url = BASE_URL.to_string() + "auth/";
            let result = make_request(&url, Method::POST, Some(body), None).await;
            match result {
                Ok(value) => {
                    token = value
                        .headers()
                        .get("authorization")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();
                    store_user_token(&user_data, &token);
                }
                Err(error) => {}
            }
        }
    }
    let mut headers = HashMap::new();
    headers.insert(String::from("Authorization"), token.as_str().to_string());
    Ok(headers)
}

#[allow(dead_code)]

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

#[allow(dead_code)]

pub async fn get_current_issue() -> Result<Vec<Company>, Error> {
    let headers = get_auth_header(None).await.unwrap();
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

#[allow(dead_code)]

pub async fn get_application_report(user: Option<User>) -> Result<Vec<CompanyApplication>, Error> {
    let headers = get_auth_header(user).await.unwrap();
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
        "size": 4,
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

pub async fn get_company_result(user: User, company_index: usize) -> Result<IPOResult, Error> {
    let headers = get_auth_header(Some(user.clone())).await.unwrap();
    let shares = get_application_report(Some(user.clone())).await.unwrap();
    let application = shares.get(company_index).unwrap();
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
