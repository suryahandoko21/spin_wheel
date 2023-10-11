use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ContentPayload{
    pub content_title:String,
    pub content_description:String,
    }