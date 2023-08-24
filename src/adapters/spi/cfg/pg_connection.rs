use std::sync::OnceLock;
use diesel::{r2d2::ConnectionManager, PgConnection};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub static CONN: OnceLock<DbPool> = OnceLock::new();
use crate::adapters::api::shared::init_global::GLOBAL_INIT;
pub fn check_connection(){
    let global_init =GLOBAL_INIT.get().unwrap();
    let database_url = &global_init["database_url"];
    let database_name = &global_init["database_name"];
    let database = format!("{}/{}", database_url, database_name);
    let manager = ConnectionManager::<PgConnection>::new(database);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    CONN.get_or_init(|| pool);


    /* nanti dikirim ke slack kalau error*/
}

