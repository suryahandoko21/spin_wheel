use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::adapters::spi::prizes::models::SpinPrizes; 
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,Serialize,Deserialize)]
#[diesel(belongs_to(SpinPrizes))]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinLists {
    pub id: i32,
    pub company_code : String,
    pub list_status : String,
    pub quantity: i32,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime,
    pub created_by : String,
    pub updated_by : String,
    pub spin_prizes_id:i32
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListWithPrize<L, P> {
    #[serde(flatten)]
    tb_list: L,
    tb_prize: P,
}

impl <L, P> From<(L,P)>for ListWithPrize<L,P>{
    fn from((l,p):(L,P)) -> Self {
        Self { tb_list: l, tb_prize: p }
    }
}