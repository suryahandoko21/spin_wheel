use crate::adapters::api::shared::expire_ticket::RequestExpire;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_companies::dsl::*;
use crate::adapters::spi::cfg::schema::tb_spin_tickets::dsl::*;
use crate::adapters::spi::spintickets::models::SpinTickets;
use crate::adapters::spi::used::post_expire::post_expire;
use local_ip_address::local_ip;

use diesel::prelude::*;
pub async fn send_ticket_expired_be() {
    let local_ip = local_ip();
    let ip = local_ip.unwrap().to_string();
    let result: Result<Vec<SpinTickets>, _> = tb_spin_tickets
        .filter(status.eq("EXPIRED"))
        .filter(send_be.eq(false))
        .load::<SpinTickets>(&mut CONN.get().unwrap().get().expect("cant connect database"));
    for i in result.iter() {
        for data in i.iter() {
            let request_expire = RequestExpire {
                uuid: data.ticket_uuid.to_string(),
                status: "EXPIRED".to_string(),
                ipAddress: ip.to_string(),
            };
            let url_address = tb_companies
                .filter(companies_code.eq(data.company_code.to_string()))
                .select(companies_address)
                .get_result::<String>(
                    &mut CONN.get().unwrap().get().expect("cant connect database"),
                );
            let addr = url_address.unwrap().to_string();
            let post_expires = post_expire(request_expire, addr).await;
            if post_expires {
                let _update_used =
                    diesel::update(tb_spin_tickets.filter(ticket_uuid.eq(&data.ticket_uuid)))
                        .set(send_be.eq(true))
                        .execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
            }
        }
    }
}
