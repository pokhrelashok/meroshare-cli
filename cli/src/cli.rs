extern crate prettytable;

use async_recursion::async_recursion;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use meroshare::{
    CompanyApplication, IPOAppliedResult, IPOResult, Meroshare, Portfolio, CURR_FORMAT,
};
use prettytable::{color, row, Cell, Row};
use prettytable::{Attr, Table};
use std::fs::File;
use std::io::Read;
use std::io::{self, Error, Write};
use std::path::Path;
use std::{env, vec};
use thousands::Separable;
use tokio::sync::{Mutex, MutexGuard};

use meroshare::user::User;
enum Action {
    ListOpenShares,
    ListResultShares,
    ViewPortfolio,
    ViewTransactions,
    CalculateProfit,
}

pub struct Handler {
    meroshare: Meroshare,
}
lazy_static! {
    pub static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![]);
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            meroshare: Meroshare::new(),
        }
    }

    pub async fn handle(&mut self) {
        let args: Vec<String> = env::args().collect();
        #[allow(unused_assignments)]
        let mut directory_path = String::new();
        if let Some(dir_path) = args.get(1) {
            directory_path = dir_path.to_string();
        } else {
            if self.users_json_exists("users.json") {
                directory_path = String::from("users.json");
            } else {
                print!("Path to the users JSON file? ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                if input.trim().is_empty() {
                    println!("Invalid file path!");
                    // self.handle().await;
                } else {
                    directory_path = input.trim().to_owned();
                }
            }
        }
        {
            let mut user_guard = USERS.lock().await;
            let mut users = self.get_users(directory_path.as_str()).unwrap();
            user_guard.append(&mut users);
        }
        self.handle_menu().await;
    }
    #[async_recursion]
    pub async fn handle_menu(&mut self) {
        let user_guard = USERS.lock().await;
        loop {
            let action = self.print_menu();
            match action {
                Ok(action) => match action {
                    Action::ListOpenShares => {
                        self.list_open_shares(&user_guard).await;
                    }
                    Action::ListResultShares => {
                        self.list_results(&user_guard).await;
                    }
                    Action::ViewPortfolio => {
                        self.view_portfolio(&user_guard).await;
                    }
                    Action::ViewTransactions => {
                        self.view_transactions(&user_guard).await;
                    }
                    Action::CalculateProfit => {
                        self.calculate_gains(&user_guard).await;
                    }
                },
                Err(_) => {
                    Handler::invalid_choice();
                    continue;
                }
            }
            print!("Press (m) to show menu: ");
            io::stdout().flush().unwrap();
            let char = Handler::read_single_character().unwrap();
            if char != 'm' {
                break;
            }
        }
    }
    fn users_json_exists(&self, file_path: &str) -> bool {
        if Path::new(file_path).exists() {
            true
        } else {
            false
        }
    }

    pub fn get_users(&self, path: &str) -> Result<Vec<User>, Error> {
        let mut file = File::open(path).expect("Invalid file path");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        let users: Vec<User> = serde_json::from_str(&contents).expect("Invalid JSON");
        Ok(users)
    }
    fn print_menu(&mut self) -> Result<Action, String> {
        println!("1. List Open Shares");
        println!("2. Check Share Result");
        println!("3. View Portfolio");
        println!("4. View Transactions");
        println!("5. Calculate Profit/Loss");
        print!("Choose an action? ");
        io::stdout().flush().unwrap();

        let input = Handler::read_single_character().unwrap();
        match input {
            '1' => Ok(Action::ListOpenShares),
            '2' => Ok(Action::ListResultShares),
            '3' => Ok(Action::ViewPortfolio),
            '4' => Ok(Action::ViewTransactions),
            '5' => Ok(Action::CalculateProfit),
            _ => Err("Invalid Selection".to_string()),
        }
    }
    fn read_single_character() -> io::Result<char> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() != "" {
                    Ok(input.trim().chars().nth(0).unwrap())
                } else {
                    Err(Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))
                }
            }
            Err(error) => Err(error),
        }
    }

    #[async_recursion]
    async fn list_open_shares<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        let user = users.get(0).unwrap();
        let shares = self.meroshare.get_current_issue(user).await.unwrap();
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
            self.fill_share(shares.get(sn - 1).unwrap().company_share_id, sn - 1, users)
                .await;
        } else {
            Handler::invalid_choice();
            self.list_open_shares(users).await;
        }
    }

    async fn fill_share<'a>(&mut self, id: i32, index: usize, users: &MutexGuard<'a, Vec<User>>) {
        let user = users.get(0).unwrap();
        let prospectus = self
            .meroshare
            .get_company_prospectus(user, id)
            .await
            .unwrap();
        prospectus.print_table();
        print!("Are you sure you want to fill this share(y/n)? ");

        io::stdout().flush().unwrap();
        let input = Handler::read_single_character().unwrap();
        let mut results: Vec<IPOAppliedResult> = vec![];
        if input != 'y' && input != 'n' {
            print!("Invalid Selection");
            return ();
        } else if input == 'n' {
            return ();
        }

        let bar = ProgressBar::new(users.len() as u64);
        for user in users.iter() {
            results.push(self.meroshare.apply_share(user, index).await.unwrap());
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
    async fn list_results<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        let user = users.get(0).unwrap();
        let shares = self.meroshare.get_application_report(user).await.unwrap();
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
        let input = Handler::read_single_character().unwrap();
        let sn = input.to_digit(10).unwrap() as usize;
        if sn > 0 && sn <= shares.len() {
            self.check_result(shares.get(sn - 1).unwrap(), sn - 1, users)
                .await;
        }
    }

    async fn check_result<'a>(
        &mut self,
        company: &CompanyApplication,
        index: usize,
        users: &MutexGuard<'a, Vec<User>>,
    ) {
        let bar = ProgressBar::new(users.len() as u64);
        let mut results: Vec<IPOResult> = vec![];
        for user in users.iter() {
            results.push(
                self.meroshare
                    .get_company_result(user, index)
                    .await
                    .unwrap(),
            );
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

    #[async_recursion]
    pub async fn view_portfolio<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        match self.select_user(users) {
            Some(sn) => {
                let user = users.get(sn - 1).unwrap();
                let portfolio = self.meroshare.get_portfolio(user).await.unwrap();
                portfolio.print_table(user);
            }
            None => {
                self.view_portfolio(users).await;
            }
        }
    }

    fn select_user<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) -> Option<usize> {
        self.print_users(users);
        print!("Choose User: ");
        io::stdout().flush().unwrap();
        let sn = Handler::read_number().unwrap();
        if sn > 0 && sn <= users.len() {
            return Some(sn);
        } else {
            Handler::invalid_choice();
            self.select_user(users);
        }
        return None;
    }

    pub fn print_users<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("S.N.").with_style(Attr::Bold),
            Cell::new("Name").with_style(Attr::Bold),
        ]));
        for (i, _) in users.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new((i + 1).to_string().as_str()),
                Cell::new(users.get(i).unwrap().name.as_str()),
            ]));
        }
        table.printstd();
    }
    fn read_number() -> io::Result<usize> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() != "" {
                    Ok(input.trim().parse::<usize>().unwrap())
                } else {
                    Err(Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))
                }
            }
            Err(error) => Err(error),
        }
    }
    pub async fn view_transactions<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        match self.select_user(users) {
            Some(sn) => {
                let user = users.get(sn - 1).unwrap();
                let transactions = self.meroshare.get_transactions(user).await.unwrap();
                transactions.print_table(user);
            }
            None => todo!(),
        }
    }

    async fn calculate_gains<'a>(&mut self, users: &MutexGuard<'a, Vec<User>>) {
        println!("1. Family");
        print!("Choose a tag? ");
        io::stdout().flush().unwrap();
        let input = Handler::read_single_character().unwrap();
        if input != '1' {
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
            let portfolio = self.meroshare.get_portfolio(user).await.unwrap();
            portfolios.push(portfolio);
            bar.inc(1);
        }
        bar.finish_and_clear();
        let mut prev_total: f64 = 0.00;
        let mut now_total: f64 = 0.00;
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
                "Total Profit Today"
            } else if now_total < prev_total {
                "Total Loss Today"
            } else {
                "Balance"
            })
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::WHITE)),
            Cell::new(
                (now_total - prev_total)
                    .abs()
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

    fn invalid_choice() {
        println!("Invalid choice!");
        io::stdout().flush().unwrap();
    }
}
