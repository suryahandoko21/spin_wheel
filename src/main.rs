use std::env;
use std::net::TcpListener;
use awc::Client;
use spin_wheel::adapters::api::shared::init_global::set_global_init;

use spin_wheel::adapters::spi::cfg::pg_connection::check_connection;
// use spin_wheel::adapters::spi::cron::crons::cron_all;
use spin_wheel::run;
use std::thread;
// use serde_json::{Result, Value};

// use my_crate::MY_VAR;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    #[allow(non_snake_case)]
    let client = Client::default();

    let res = client.get("https://query.lidoapi.com/companies/all")
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await;
      
    let body = res.ok().unwrap().body().await.ok().unwrap();
    // println!("oke{:?}",body);
    let _vec = body.to_vec();
    
    // println!("vector{:?}",vec);
    // let sparkle_heart = str::from_utf8(&vec);
    // let val = 
    // println!("ssss{:?}",sparkle_heart);
    // println!("dsdsd{:?}",sparkle_heart.ok().unwrap());
    // let  _val:  = serde_json::fr;
    // println!("{:?}",val);
    // while let Some(i) = v.as_array(){
    //     println!("value{:?}",i);
    // }
    // for x in val {
    //     println!("str{:?}",x);
    // }





    
    set_global_init();
    thread::spawn(move||{
        /* new thread fro cron */
        // cron_all();
    });
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();
    check_connection();
    let port = dotenv::var("PORT").expect("Failed to fetch port in .env");
    let listener = TcpListener::bind("0.0.0.0:".to_owned()+&port).expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
   
    run(listener, &database_name)?.await

}
