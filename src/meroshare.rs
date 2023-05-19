#[path = "request.rs"]
mod request;
use request::make_request;
use reqwest::Method;
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

const BASE_URL: &str = "https://backend.cdsc.com.np/api";
pub async fn init() {
    let auth = login().await;
}
async fn login() -> String {
    let url = BASE_URL.to_string() + "/meroShare/auth/";
    let mut body = HashMap::new();
    body.insert("clientId", "160");
    body.insert("username", "00015818");
    body.insert("password", "@143Meroshare");

    let result = make_request(&url, Method::POST, Some(body), None).await;
    match result {
        Ok(value) => {
            print!("{:?}", value.headers().get("authorization").unwrap());
            value
                .headers()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        }
        Err(error) => String::from("Fatgaya"),
    }
}
