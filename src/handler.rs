extern crate prettytable;
#[path = "meroshare.rs"]
mod meroshare;
#[path = "company.rs"]
use crate::meroshare::get_current_issue;
use prettytable::row;
use std::io::{self, Error};
#[macro_use]
use prettytable::{Cell, Row, Table};

enum Action {
    ViewOpenShare,
    ViewShareResult,
    FillShare,
}

pub async fn handle() {
    let action = print_menu();
    match action {
        Ok(action) => match action {
            Action::ViewOpenShare => {
                list_open_shares().await;
            }
            Action::ViewShareResult => todo!(),
            Action::FillShare => todo!(),
        },
        Err(_) => {
            println!("Invalid Choice!");
        }
    }
}

fn print_menu() -> Result<Action, String> {
    println!("1. View Open Share");
    println!("2. View Share Result");
    println!("3. Fill a Share");
    println!("Choose an action? ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim() {
        "1" => Ok(Action::ViewOpenShare),
        "2" => Ok(Action::ViewShareResult),
        "3" => Ok(Action::FillShare),
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
    println!("Which Share do you want to fill? ");
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

async fn fill_share(id: i32) {}
