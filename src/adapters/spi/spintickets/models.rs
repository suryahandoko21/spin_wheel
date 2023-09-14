use diesel::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinTickets {
    pub id: i32,
    pub user_uuid : String,
    pub userid: String,
    pub username : String, 
    pub ticket_id : i32,
    pub ticket_uuid : String,
    pub status : String,
    pub pointrule_id: i32,
    pub expired_date: String,
    pub pointrule_name:String,
    pub ticket_number:String,
    pub expired_type:String,
    pub expired_value:i32,
    pub created_date:String,
    pub is_payment_gateway :bool
}

#[derive(Queryable, Selectable, Debug, PartialEq,Serialize,Deserialize,Insertable,QueryableByName)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinTicketsToDb {
    pub user_uuid : String,
    pub userid: String,
    pub username : String, 
    pub ticket_id : i32,
    pub ticket_uuid : String,
    pub status : String,
    pub pointrule_id: i32,
    pub expired_date: String,
    pub pointrule_name:String,
    pub ticket_number:String,
    pub expired_type:String,
    pub expired_value:i32,
    pub created_date:String,
    pub is_payment_gateway :bool
}