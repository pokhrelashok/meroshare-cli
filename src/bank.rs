use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Bank {
    code: String,
    id: u32,
    name: String,
}
