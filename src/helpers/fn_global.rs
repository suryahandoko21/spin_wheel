use chrono::NaiveDateTime;
pub fn convert_str_to_timestamp(date_str: String) -> NaiveDateTime {
    let naive_datetime =
        NaiveDateTime::parse_from_str(&date_str.to_string(), "%Y-%m-%d %H:%M:%S").unwrap();
    return naive_datetime;
}

pub fn hello() {
    println!("Hello, my honey buny sweety!");
}
pub fn some_error(msg: &str) -> ! {
    // Note the `-> !` here
    eprintln!("Error: {}", msg);
    panic!();
}
pub fn some_error2(msg: String) -> String {
    return msg.to_string();
}
