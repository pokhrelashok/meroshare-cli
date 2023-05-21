extern crate prettytable;

use crate::ipo_result::IPOResult;
use crate::meroshare::get_current_issue;
use crate::meroshare::{get_application_report, get_company_result};
use prettytable::row;
use std::io::{self, Error};
#[macro_use]
use prettytable::{Cell, Row, Table};

use crate::company::CompanyApplication;
use crate::user::get_users;

enum Action {
    ListOpenShares,
    ListResultShares,
}

pub async fn handle() {
    let action = print_menu();
    match action {
        Ok(action) => match action {
            Action::ListOpenShares => {
                list_open_shares().await;
            }
            Action::ListResultShares => {
                list_results().await;
            }
        },
        Err(_) => {
            println!("Invalid Choice!");
        }
    }
}

fn print_menu() -> Result<Action, String> {
    println!("1. List Open Shares");
    println!("2. Check Share Result");
    println!("Choose an action? ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim() {
        "1" => Ok(Action::ListOpenShares),
        "2" => Ok(Action::ListResultShares),
        _ => Err("Invalid Selection".to_string()),
    }
}

async fn list_open_shares() {
    let shares = get_current_issue().await.unwrap();
    let mut table = Table::new();
    table.add_row(row!["S.N.", "Company Name", "Type", "Close Date"]);
    for (i, share) in shares.iter().enumerate() {
        table.add_row(row![
            i + 1,
            share.company_name,
            share.share_type_name,
            share.issue_close_date
        ]);
    }
    table.printstd();
    println!("Which share do you want to fill? ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{:}", input);
    let sn = input.trim().parse::<usize>().unwrap();
    if sn > 0 && sn <= shares.len() {
        fill_share(shares.get(sn).unwrap().company_share_id).await;
    }
}

async fn list_results() {
    let shares = get_application_report(None).await.unwrap();
    let mut table = Table::new();
    table.add_row(row!["S.N.", "Company Name", "Type", "Status"]);
    for (i, share) in shares.iter().enumerate() {
        table.add_row(row![
            i + 1,
            share.company_name,
            share.share_type_name,
            share.status_name
        ]);
    }
    table.printstd();
    println!("Which share's result do you want to check? ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{:}", input);
    let sn = input.trim().parse::<usize>().unwrap();
    if sn > 0 && sn <= shares.len() {
        check_result(sn).await;
    }
}

async fn fill_share(id: i32) {}

async fn check_result(index: usize) {
    let users = get_users();
    let mut results: Vec<IPOResult> = vec![];
    for (index, user) in users.iter().enumerate() {
        results.push(get_company_result(user.clone(), index).await.unwrap());
    }

    let mut table = Table::new();
    table.add_row(row!["S.N.", "Name", "Status"]);
    for (i, result) in results.iter().enumerate() {
        table.add_row(row![i + 1, result.status, result.status]);
    }
}
