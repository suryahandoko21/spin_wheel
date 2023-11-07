use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_tickets::dsl::*;
use crate::adapters::spi::spintickets::models::SpinTickets;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
pub async fn check_ticket_expired_be() {
    let result: Result<Vec<SpinTickets>, _> = tb_spin_tickets
        .filter(status.eq("AVAILABLE"))
        .filter(expired_date.ne(""))
        .load::<SpinTickets>(&mut CONN.get().unwrap().get().expect("cant connect database"));
    for i in result.iter() {
        for data in i.iter() {
            let t_uuid = data.ticket_uuid.to_string();
            let utc_now = Utc::now() + Duration::hours(7);
            let naive_now: NaiveDateTime = utc_now.naive_utc();
            let expired = data.expired_date.to_string();
            let format_str = "%Y-%m-%d %H:%M:%S";
            let expire_date = NaiveDateTime::parse_from_str(&expired, format_str).unwrap();
            if expire_date <= naive_now {
                let _expired_used =
                    diesel::update(tb_spin_tickets.filter(ticket_uuid.eq(t_uuid.to_string())))
                        .set(status.eq("EXPIRED"))
                        .execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
            }
        }
    }
}
