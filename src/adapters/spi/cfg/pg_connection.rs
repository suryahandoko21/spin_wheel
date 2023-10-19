use diesel::{r2d2::ConnectionManager, PgConnection};
use std::sync::OnceLock;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub static CONN: OnceLock<DbPool> = OnceLock::new();
use crate::adapters::api::{
    shared::init_global::GLOBAL_INIT, slack::push_notif::push_notification,
};
pub async fn check_connection() {
    let global_init = GLOBAL_INIT.get().unwrap();
    let database_url = &global_init["database_url"];
    let database_name = &global_init["database_name"];
    let database = format!("{}/{}", database_url, database_name);
    let manager = ConnectionManager::<PgConnection>::new(database);
    let pool = r2d2::Pool::builder().build(manager);
    if pool.is_err() {
        let _x = push_notification("❌ Database not connected !!".to_string()).await;
    } else {
        CONN.get_or_init(|| pool.expect("Failed connection"));
    }
}
