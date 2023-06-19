use std::time::{SystemTime};
#[derive(Debug, Clone)]
pub struct SpinListsEntity{
    pub list_id: i32,
    pub company_code : String,
    pub list_status : String,
    pub quantity: i32,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime,
    pub created_by : String,
    pub updated_by : String,
    pub spin_prizes_id:i32
}

impl SpinListsEntity {
    pub fn new(
        list_id: i32,
        company_code: String,
        list_status: String,
        quantity: i32,
        created_at : SystemTime, 
        updated_at : SystemTime,
        created_by : String,
        updated_by : String,
        spin_prizes_id:i32
    )->Self{
        SpinListsEntity{
            list_id,
            company_code,
            list_status,
            quantity,
            created_at,
            updated_at,
            created_by ,
            updated_by ,
            spin_prizes_id
        }

    }
}
