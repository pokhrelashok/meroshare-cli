use reqwest::Error;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

use crate::bank::Bank;
use crate::bank::BankDetails;
use crate::company::Company;
use crate::company::CompanyApplication;
use crate::company::Prospectus;
use crate::file::get_user_stored_token;
use crate::file::store_user_token;
use crate::ipo::IPOAppliedResult;
use crate::ipo::IPOResult;
use crate::portfolio::Portfolio;
use crate::request::make_request;
use crate::transaction::TransactionView;
use crate::user::get_users;
use crate::user::User;
use crate::user::UserDetails;

const PORTFOLIO_URL: &str = "https://backend.cdsc.com.np/api/meroShareView/";
const MERO_SHARE_URL: &str = "https://backend.cdsc.com.np/api/meroShare/";

async fn get_auth_header(user: Option<&User>) -> Result<HashMap<String, String>, Error> {
    let user_data =
        user.unwrap_or_else(|| Box::leak(Box::new(get_users().get(0).unwrap().to_owned())));
    let mut token: String = String::new();
    match get_user_stored_token(&user_data.username) {
        Some(t) => {
            token = t;
        }
        None => {
            let body = json!({
                "clientId":user_data.dp_id,
                "username":user_data.username,
                "password":user_data.password,
            });
            let url = MERO_SHARE_URL.to_string() + "auth/";
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
                Err(_error) => {}
            }
        }
    }
    let mut headers = HashMap::new();
    headers.insert(String::from("Authorization"), token.as_str().to_string());
    Ok(headers)
}

#[allow(dead_code)]

pub async fn get_banks(user: &User) -> Result<Vec<Bank>, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "bank/";
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let banks: Vec<Bank> = value.json().await?;
            Ok(banks)
        }
        Err(error) => Err(error),
    }
}

pub async fn get_bank_details(id: u32, user: &User) -> Result<BankDetails, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "bank/" + id.to_string().as_str();
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let banks: BankDetails = value.json().await?;
            Ok(banks)
        }
        Err(error) => Err(error),
    }
}
pub async fn get_user_details(user: &User) -> Result<UserDetails, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "ownDetail/";
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let user: UserDetails = value.json().await?;
            Ok(user)
        }
        Err(error) => Err(error),
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponseCurrentIssue {
    object: Vec<Company>,
}

#[allow(dead_code)]

pub async fn get_current_issue(user: Option<&User>) -> Result<Vec<Company>, Error> {
    let headers = get_auth_header(user).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "companyShare/currentIssue/";
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

pub async fn get_application_report(user: Option<&User>) -> Result<Vec<CompanyApplication>, Error> {
    let headers = get_auth_header(user).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "applicantForm/active/search/";
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

pub async fn get_company_result(user: &User, company_index: usize) -> Result<IPOResult, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let shares = get_application_report(Some(user)).await.unwrap();
    let application = shares.get(company_index).unwrap();
    let url = MERO_SHARE_URL.to_string()
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

pub async fn get_company_prospectus(id: i32) -> Result<Prospectus, Error> {
    let headers = get_auth_header(None).await.unwrap();
    let url = MERO_SHARE_URL.to_string() + "active/" + (id).to_string().as_str();
    let result = make_request(&url, Method::GET, None, Some(headers)).await;
    match result {
        Ok(value) => {
            let result: Prospectus = value.json().await?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}

pub async fn get_portfolio(user: &User) -> Result<Portfolio, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let user_details = get_user_details(&user).await.unwrap();
    let url = PORTFOLIO_URL.to_string() + "myPortfolio";
    let body = json!({
        "clientCode":user.dp,
        "demat":[user_details.demat],
        "page":1,
        "size":500,
        "sortAsc":true,
        "sortBy":"script",
    });
    let result = make_request(&url, Method::POST, Some(body), Some(headers)).await;
    match result {
        Ok(value) => {
            let result: Portfolio = value.json().await?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}

pub async fn get_transactions(user: &User) -> Result<TransactionView, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let user_details = get_user_details(&user).await.unwrap();
    let url = PORTFOLIO_URL.to_string() + "myTransaction";
    let body = json!({
        "boid":user_details.demat,
        "clientCode":user.dp,
        "page":1,
        "requestTypeScript":false,
        "script":null,
        "size":200,
    });
    let result = make_request(&url, Method::POST, Some(body), Some(headers)).await;
    match result {
        Ok(value) => {
            let result: TransactionView = value.json().await?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}

pub async fn apply_share(user: &User, company_index: usize) -> Result<IPOAppliedResult, Error> {
    let headers = get_auth_header(Some(user)).await.unwrap();
    let shares = get_current_issue(Some(user)).await.unwrap();
    let banks = get_banks(user).await.unwrap();
    let bank = banks.get(user.bank_index - 1).unwrap();
    let bank_details = get_bank_details(bank.id, user).await.unwrap();
    let user_details = get_user_details(user).await.unwrap();
    let opening = shares.get(company_index).unwrap();
    let url = MERO_SHARE_URL.to_string() + "applicantForm/share/apply/";
    let body = json!({
        "accountBranchId":bank_details.account_branch_id,
        "accountNumber":bank_details.account_number,
        "appliedKitta":10,
        "bankId":bank.id,
        "boid":user_details.boid,
        "companyShareId":opening.company_share_id,
        "crnNumber":user.crn,
        "customerId":bank_details.id,
        "demat":user_details.demat,
        "transactionPIN":user.pin,
    });
    let result = make_request(&url, Method::POST, Some(body), Some(headers)).await;
    match result {
        Ok(value) => {
            let result: IPOAppliedResult = value.json().await?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}
