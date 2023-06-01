use prettytable::{Table, Cell, Row, Attr};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Portfolio {
    #[serde(rename = "totalItems")]
    pub total_items: f32,
    #[serde(rename = "totalValueOfLastTransPrice")]
    pub total_value_of_last_trans_price: f32,
    #[serde(rename = "totalValueOfPrevClosingPrice")]
    pub total_value_of_prev_closing_price: f32,
    #[serde(rename = "meroShareMyPortfolio")]
    pub items:Vec<PortfolioItem>,
}
impl Portfolio {
    pub fn print(&self) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Script").with_style(Attr::Bold),
            Cell::new("Current Balance").with_style(Attr::Bold),
            Cell::new("LTP").with_style(Attr::Bold),
            Cell::new("Value of Previous Closing Price").with_style(Attr::Bold),
            Cell::new("Value of LTP").with_style(Attr::Bold),
        ]));

        for item in &self.items {
            table.add_row(Row::new(vec![
                Cell::new(&item.script),
                Cell::new(&item.current_balance.to_string()),
                Cell::new(&item.last_transaction_price),
                Cell::new(&item.value_of_prev_closing_price.to_string()),
                Cell::new(&item.value_of_last_trans_price.to_string()),
            ]));
        }
        table.add_row(Row::new(vec![
          Cell::new(""),
          Cell::new(""),
          Cell::new(""),
          Cell::new(&self.total_value_of_prev_closing_price.to_string()).with_style(Attr::Bold),
          Cell::new(&self.total_value_of_last_trans_price.to_string()).with_style(Attr::Bold),
        ]));
        table.printstd();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PortfolioItem {
    #[serde(rename = "currentBalance")]
    pub current_balance: f32,
    #[serde(rename = "lastTransactionPrice")]
    pub last_transaction_price: String,
    #[serde(rename = "previousClosingPrice")]
    pub previous_closing_price: String,
    pub script: String,
    #[serde(rename = "scriptDesc")]
    pub script_desc: String,
    #[serde(rename = "valueAsOfLastTransactionPrice")]
    pub value_as_of_last_transaction_price: String,
    #[serde(rename = "valueAsOfPreviousClosingPrice")]
    pub value_as_of_previous_closing_price: String,
    #[serde(rename = "valueOfLastTransPrice")]
    pub value_of_last_trans_price: f32,
    #[serde(rename = "valueOfPrevClosingPrice")]
    pub value_of_prev_closing_price: f64,
}