use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq,Serialize,Deserialize,Insertable)]

#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinTickets {
    pub id: i32,
    pub user_uuid : String,
    pub ruleid : String,
    pub userid: String,
    pub username : String, 
    pub ticket_id : i32,
    pub ticket_uuid : String,
    pub status : String,
    pub pointrule_id: i32,
    pub expired_date: String
}

#[derive(Queryable, Selectable, Debug, PartialEq,Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinTicketsToDb {
    pub user_uuid : String,
    pub ruleid : String,
    pub userid: String,
    pub username : String, 
    pub ticket_id : i32,
    pub ticket_uuid : String,
    pub status : String,
    pub pointrule_id: i32,
    pub expired_date: String
}