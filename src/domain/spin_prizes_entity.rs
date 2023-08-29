use crate::adapters::spi::companies::models::Companies;

#[derive(Debug, Clone)]
pub struct SpinPrizesEntity {
    pub prize_id: i32,
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32,
    pub prize_money :i32,
    pub companies_id : String,
    pub percentage:i32,
    pub prize_image:String
}

impl SpinPrizesEntity {
    pub fn new(
        prize_id: i32,
        prize_name: String,
        prize_note: String,
        prize_category: String,
        prize_amount: i32,
        prize_money: i32,
        companies_id: String,
        percentage:i32,
        prize_image:String
    )->Self{
         SpinPrizesEntity{
            prize_id,
            prize_name,
            prize_note,
            prize_category,
            prize_amount,
            prize_money,
            companies_id,
            percentage,
            prize_image
        }

    }
    
}

#[derive(Debug)]
pub struct SpinPrizesCompaniesEntity {
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32,
    pub companies : Companies,
    pub percentage: i32,
}

impl SpinPrizesCompaniesEntity {
    pub fn new(
        prize_name: String,
        prize_note: String,
        prize_category: String,
        prize_amount: i32,
        companies : Companies,
        percentage:i32,
    )->Self{
        SpinPrizesCompaniesEntity{
            prize_name,
            prize_note,
            prize_category,
            prize_amount,
            companies,
            percentage
     
        }

    }    
}

