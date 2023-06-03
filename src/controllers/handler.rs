extern crate prettytable;

use std::io::{self, Write};
use std::vec;

use crate::company::CompanyApplication;
use crate::currency::CURR_FORMAT;
use crate::ipo::{IPOAppliedResult, IPOResult};
use crate::meroshare::{
    apply_share, get_application_report, get_company_prospectus, get_company_result,
    get_transactions,
};
use crate::meroshare::{get_current_issue, get_portfolio};
use crate::portfolio::Portfolio;
use crate::user::{print_users, User};
use async_recursion::async_recursion;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use prettytable::{color, row, Cell, Row};
use prettytable::{Attr, Table};
use std::fs::File;
use std::io::Error;
use std::io::Read;
use thousands::Separable;
use tokio::sync::{Mutex, MutexGuard};

enum Action {
    ListOpenShares,
    ListResultShares,
    ViewPortfolio,
    ViewTransactions,
    CalculateProfit,
}

lazy_static! {
    pub static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![]);
}

#[async_recursion]
pub async fn handle(path: &str) {
    let mut users_guard = USERS.lock().await;
    let mut users = get_users(path).unwrap();
    users_guard.append(&mut users);
    let action = print_menu();
    match action {
        Ok(action) => match action {
            Action::ListOpenShares => {
                list_open_shares(&users_guard).await;
            }
            Action::ListResultShares => {
                list_results(&users_guard).await;
            }
            Action::ViewPortfolio => {
                view_portfolio(&users_guard).await;
            }
            Action::ViewTransactions => {
                view_transactions(&users_guard).await;
            }
            Action::CalculateProfit => {
                calculate_gains(&users_guard).await;
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
    println!("3. View Portfolio");
    println!("4. View Transactions");
    println!("5. Calculate Profit/Loss");
    print!("Choose an action? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim() {
        "1" => Ok(Action::ListOpenShares),
        "2" => Ok(Action::ListResultShares),
        "3" => Ok(Action::ViewPortfolio),
        "4" => Ok(Action::ViewTransactions),
        "5" => Ok(Action::CalculateProfit),
        _ => Err("Invalid Selection".to_string()),
    }
}

async fn list_open_shares<'a>(users: &MutexGuard<'a, Vec<User>>) {
    let user = users.get(0).unwrap();
    let shares = get_current_issue(user).await.unwrap();
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
        fill_share(shares.get(sn - 1).unwrap().company_share_id, sn - 1, users).await;
    }
}

async fn list_results<'a>(users: &MutexGuard<'a, Vec<User>>) {
    let user = users.get(0).unwrap();
    let shares = get_application_report(user).await.unwrap();
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
        check_result(shares.get(sn - 1).unwrap(), sn - 1, users).await;
    }
}

async fn fill_share<'a>(id: i32, index: usize, users: &MutexGuard<'a, Vec<User>>) {
    let user = users.get(0).unwrap();
    let prospectus = get_company_prospectus(user, id).await.unwrap();
    prospectus.print();
    print!("Are you sure you want to fill this share(y/n)? ");

    io::stdout().flush().unwrap();
    let mut input = String::new();
    let mut results: Vec<IPOAppliedResult> = vec![];
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let selection = input.chars().nth(0).unwrap();
    if selection != 'y' && selection != 'n' {
        print!("Invalid Selection");
        return ();
    } else if selection == 'n' {
        return ();
    }

    let bar = ProgressBar::new(users.len() as u64);
    for user in users.iter() {
        results.push(apply_share(user, index).await.unwrap());
        bar.inc(1);
    }
    bar.finish_and_clear();

    let mut table = Table::new();
    let row = Row::new(vec![Cell::new(
        format!("Applied {}", prospectus.company_name).as_str(),
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

async fn check_result<'a>(
    company: &CompanyApplication,
    index: usize,
    users: &MutexGuard<'a, Vec<User>>,
) {
    let bar = ProgressBar::new(users.len() as u64);
    let mut results: Vec<IPOResult> = vec![];
    for user in users.iter() {
        results.push(get_company_result(user, index).await.unwrap());
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
                if result.status.contains("Not") || result.status.contains("Rejected") {
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

pub async fn view_portfolio<'a>(users: &MutexGuard<'a, Vec<User>>) {
    match select_user(users) {
        Some(sn) => {
            let user = users.get(sn - 1).unwrap();
            let portfolio = get_portfolio(user).await.unwrap();
            portfolio.print(user);
        }
        None => todo!(),
    }
}

pub async fn view_transactions<'a>(users: &MutexGuard<'a, Vec<User>>) {
    match select_user(&users) {
        Some(sn) => {
            let user = users.get(sn - 1).unwrap();
            let transactions = get_transactions(user).await.unwrap();
            transactions.print(user);
        }
        None => todo!(),
    }
}

fn select_user<'a>(users: &MutexGuard<'a, Vec<User>>) -> Option<usize> {
    print_users(users);
    print!("Choose User: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let sn = input.trim().parse::<usize>().unwrap();
    if sn > 0 && sn <= users.len() {
        return Some(sn);
    }
    println!("Invalid choise!");
    return None;
}

async fn calculate_gains<'a>(users: &MutexGuard<'a, Vec<User>>) {
    println!("1. Family");
    print!("Choose a tag? ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim() != "1" {
        return;
    }
    let search = "family".to_string();
    let tag_users: Vec<&User> = users
        .iter()
        .filter(|user| user.tags.contains(&search))
        .collect();

    let mut portfolios: Vec<Portfolio> = vec![];
    let bar = ProgressBar::new(tag_users.len() as u64);
    for user in tag_users.iter() {
        let portfolio = get_portfolio(user).await.unwrap();
        portfolios.push(portfolio);
        bar.inc(1);
    }
    bar.finish_and_clear();
    let mut prev_total: f32 = 0.00;
    let mut now_total: f32 = 0.00;
    for portfolio in portfolios.iter() {
        prev_total += portfolio.total_value_of_prev_closing_price;
        now_total += portfolio.total_value_of_last_trans_price;
    }

    let mut table = Table::new();
    let row = Row::new(vec![Cell::new("Portfolio Calculations")
        .with_hspan(2)
        .style_spec("cb")]);
    table.add_row(row);
    table.add_row(Row::new(vec![
        Cell::new("Previous Closing Price"),
        Cell::new(
            prev_total
                .separate_by_policy(CURR_FORMAT)
                .to_string()
                .as_str(),
        ),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Latest Closing Price"),
        Cell::new(
            now_total
                .separate_by_policy(CURR_FORMAT)
                .to_string()
                .as_str(),
        ),
    ]));
    let is_balanced = now_total == prev_total;
    let color = match now_total > prev_total {
        true => color::GREEN,
        false => color::RED,
    };
    table.add_row(Row::new(vec![
        Cell::new(if now_total > prev_total {
            "Total Profit"
        } else if now_total < prev_total {
            "Total Loss"
        } else {
            "Balance"
        })
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color::WHITE)),
        Cell::new(
            now_total
                .separate_by_policy(CURR_FORMAT)
                .to_string()
                .as_str(),
        )
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(match is_balanced {
            true => color::WHITE,
            false => color,
        })),
    ]));
    table.printstd();
}

pub fn get_users(path: &str) -> Result<Vec<User>, Error> {
    let mut file = File::open(path).expect("Invalid file path");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let users: Vec<User> = serde_json::from_str(&contents).expect("Invalid JSON");
    Ok(users)
}
