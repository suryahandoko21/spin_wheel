use diesel::{prelude::*};
use async_trait::async_trait;
use crony::{Job, Runner, Schedule};
use std::thread;
use std::time::Duration;


use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::mem;
use std::time::SystemTime;
use diesel::dsl::count;
use diesel::sql_types::{Integer, Text};
use diesel::{prelude::*};

use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::sql_query;
use crate::adapters::api::shared::enum_response::Option;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::{TicketResponse, SpinAvailableResponse};
use crate::adapters::api::spin_lists::spin_list_payloads::SpinListPayload;
use crate::adapters::api::spin_tickets::spin_tickets_payloads::SpinTicketPayload;

use crate::adapters::spi::cfg::schema::{tb_spin_tickets, tb_spin_prizes};
use crate::adapters::spi::cfg::{schema::tb_spin_tickets::dsl::*};

use crate::adapters::spi::prizes::models::SpinPrizes;
use crate::adapters::spi::spinlist::models::SpinListsPrizes;
use crate::adapters::spi::spintickets::models::SpinTicketsToDb;
use crate::application::mappers::db_mapper::DBMapper;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::domain::spin_tickets_entity::SpinTicketsEntity;
use std::sync::Arc;

use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
// use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
impl CronEntityAbstract for ConnectionRepository {
     fn check_pending_post_be(self){
        // let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool"); 
        // let c = self.db_connection.connect();
        // println!("DASDA{:?}",c);
        // let query = tb_spin_tickets.filter(user_uuid.eq("14f17f89-ec80-4911-98a1-628ae2ca3e87")).select(count(id)).get_result::<i64>(&mut c);
        // println!("query{:?}",query);
        // println!("check pending request->");
    }
}

