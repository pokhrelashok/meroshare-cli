extern crate prettytable;

use std::io::{self, Write};

use crate::company::CompanyApplication;
use crate::file::{create_file, delete_file};
use crate::ipo_result::IPOResult;
use crate::meroshare::get_current_issue;
use crate::meroshare::{get_application_report, get_company_result};
use indicatif::ProgressBar;
use prettytable::{color, row, Cell, Row};
use prettytable::{Attr, Table};

use crate::user::get_users;

enum Action {
    ListOpenShares,
    ListResultShares,
}

pub async fn handle() {
    delete_file();
    create_file();
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
    print!("Choose an action? ");
    io::stdout().flush().unwrap();

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
    table.add_row(Row::new(vec![
        Cell::new("S.N.").with_style(Attr::Bold),
        Cell::new("Company Name").with_style(Attr::Bold),
        Cell::new("Type").with_style(Attr::Bold),
        Cell::new("Close Date").with_style(Attr::Bold),
    ]));
    for (i, share) in shares.iter().enumerate() {
        table.add_row(row![
            i + 1,
            share.company_name,
            share.share_type_name,
            share.issue_close_date
        ]);
    }
    table.printstd();
    print!("Which share do you want to fill? ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let sn = input.trim().parse::<usize>().unwrap();
    if sn > 0 && sn <= shares.len() {
        fill_share(shares.get(sn).unwrap().company_share_id).await;
    }
}

async fn list_results() {
    let shares = get_application_report(None).await.unwrap();
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("S.N.").with_style(Attr::Bold),
        Cell::new("Company Name").with_style(Attr::Bold),
        Cell::new("Type").with_style(Attr::Bold),
        Cell::new("Status").with_style(Attr::Bold),
    ]));
    for (i, share) in shares.iter().enumerate() {
        table.add_row(row![
            i + 1,
            share.company_name,
            share.share_type_name,
            share.status_name
        ]);
    }
    table.printstd();
    print!("Which share's result do you want to check? ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let sn = input.trim().parse::<usize>().unwrap();
    if sn > 0 && sn <= shares.len() {
        check_result(shares.get(sn - 1).unwrap(), sn - 1).await;
    }
}

async fn fill_share(id: i32) {}

async fn check_result(company: &CompanyApplication, index: usize) {
    let users = get_users();
    let bar = ProgressBar::new(users.len() as u64);
    let mut results: Vec<IPOResult> = vec![];
    for user in users.iter() {
        results.push(get_company_result(user.clone(), index).await.unwrap());
        bar.inc(1);
    }
    bar.finish_and_clear();
    let mut table = Table::new();
    let row = Row::new(vec![Cell::new(
        format!("Result for {}", company.company_name).as_str(),
    )
    .with_hspan(3)
    .with_style(Attr::Bold)]);
    table.add_row(row.clone());
    table.add_row(Row::new(vec![
        Cell::new("S.N.").with_style(Attr::Bold),
        Cell::new("Name").with_style(Attr::Bold),
        Cell::new("Status").with_style(Attr::Bold),
    ]));
    for (i, result) in results.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new((i + 1).to_string().as_str()),
            Cell::new(users.get(i).unwrap().name.as_str()),
            Cell::new(&result.status).with_style(Attr::ForegroundColor(
                if result.status.contains("Not") {
                    color::RED
                } else {
                    color::GREEN
                },
            )),
        ]));
    }
    table.add_row(row);
    table.printstd();
}
