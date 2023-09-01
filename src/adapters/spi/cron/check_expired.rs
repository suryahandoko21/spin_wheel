use std::sync::Arc;

use chrono::{Utc, NaiveDate};
use diesel::prelude::*;

use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::failed::models::FailedProcess;
use crate::adapters::spi::spintickets::models::SpinTickets;

use crate::adapters::spi::cfg::schema::tb_spin_tickets::dsl::*;
pub async fn check_ticket_expired_be(){

    let result:Result<Vec<SpinTickets>,_> = tb_spin_tickets.filter(status.eq("AVAILABLE")).load::<SpinTickets>(&mut CONN.get().unwrap().get().expect("cant connect database"));
    for i in result.iter(){
        for data in i.iter(){
            let ex_value: Arc<i64>= Arc::new(data.expired_value.into());
            let t_uuid = Arc::new(data.ticket_uuid.to_string());
            let d_now = Utc::now().date_naive();
            let d_created = data.created_date.to_string();
            let n_date = NaiveDate::parse_from_str(&d_created, "%Y-%m-%d").unwrap();
            let d_diff = d_now.signed_duration_since(n_date);
            let days = d_diff.num_days();
            if days >= *ex_value {
                let _expired_used = diesel::update(tb_spin_tickets.filter(ticket_uuid.eq(t_uuid.to_string()))).set(status.eq("EXPIRED")).execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
            }
       
        }
    }


    


}