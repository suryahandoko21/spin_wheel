use chrono::NaiveDateTime;
#[derive(Debug, Clone)]
pub struct SpinPromosEntity {
    pub promo_id: i32,
    pub promo_amount: i32,
    pub promo_status: String,
    pub user_id : String,
    pub username : String,
    pub point_currention_time : NaiveDateTime,
    pub expired_at :NaiveDateTime,
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
    pub created_by : String,
    pub updated_by : String,
}


impl SpinPromosEntity {
    pub fn new(
        promo_id: i32,
        promo_amount: i32,
        promo_status : String,
        user_id : String,
        username :String,
        point_currention_time : NaiveDateTime,
        expired_at :NaiveDateTime,
        created_at : NaiveDateTime, 
        updated_at : NaiveDateTime,
        created_by : String,
        updated_by : String,
    )->Self{
        SpinPromosEntity{
            promo_id,
            promo_amount,
            promo_status,
            user_id,
            username,
            point_currention_time,
            expired_at,
            created_at,
            updated_at,
            created_by,
            updated_by
        }

    }
}

