#[path = "meroshare.rs"]
mod meroshare;
#[path = "company.rs"]
use crate::meroshare::get_current_issue;
use std::io::{self, Error};

enum Action {
    ViewOpenShare,
    ViewShareResult,
    FillShare,
}
pub async fn handle() {
    let action = print_menu();
    match action {
        Ok(action) => {
            match action {
                Action::ViewOpenShare => view_open_shares().await,
                Action::ViewShareResult => todo!(),
                Action::FillShare => todo!(),
            };
        }
        Err(_) => {
            println!("Invalid Choice!");
            print_menu();
        }
    }
}

async fn view_open_shares() {
    let shares = get_current_issue().await.unwrap();
    for share in shares {
        println!(
            "{} of type {} is open until {}",
            share.company_name, share.share_type_name, share.issue_close_date
        )
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
