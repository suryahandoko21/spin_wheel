use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentPresenter {
    pub id: i32,
    pub title: String,
    pub description: String,
}
