use chrono::NaiveDateTime;
#[derive(Debug, Clone)]
pub struct SpinRewardEntity {
    pub reward_id: i32,
    pub reward_name: String,
    pub reward_note: String,
    pub reward_category: String,
    pub reward_amount: i32,
    pub reward_money :i32,
    pub reward_order:i32,
    pub companies_code : String,
    pub percentage:i32,
    pub reward_image:String,
    pub reward_status:String,
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
}

impl SpinRewardEntity {
    pub fn new(
        reward_id: i32,
        reward_name: String,
        reward_note: String,
        reward_category: String,
        reward_amount: i32,
        reward_money: i32,
        reward_order:i32,
        companies_code: String,
        percentage:i32,
        reward_image:String,
        reward_status:String,
        created_at : NaiveDateTime, 
        updated_at : NaiveDateTime,
    )->Self{
        SpinRewardEntity{
            reward_id,
            reward_name,
            reward_note,
            reward_category,
            reward_amount,
            reward_money,
            reward_order,
            companies_code,
            percentage,
            reward_image,
            reward_status,
            created_at,
            updated_at
        }
    }
    
}

// #[derive(Debug)]
// pub struct SpinRewardCompaniesEntity {
//     pub prize_name: String,
//     pub prize_note: String,
//     pub prize_category: String,
//     pub prize_amount: i32,
//     pub companies : Companies,
//     pub percentage: i32,
// }

// impl SpinRewardCompaniesEntity {
//     pub fn new(
//         prize_name: String,
//         prize_note: String,
//         prize_category: String,
//         prize_amount: i32,
//         companies : Companies,
//         percentage:i32,
//     )->Self{
//         SpinRewardCompaniesEntity{
//             prize_name,
//             prize_note,
//             prize_category,
//             prize_amount,
//             companies,
//             percentage
     
//         }

//     }    
// }

