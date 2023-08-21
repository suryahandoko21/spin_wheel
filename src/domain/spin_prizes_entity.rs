use crate::adapters::spi::companies::models::Companies;

#[derive(Debug, Clone)]
pub struct SpinPrizesEntity {
    pub prize_id: i32,
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32,
    pub companies_id : i32,
    pub percentage:i32
}

impl SpinPrizesEntity {
    pub fn new(
        prize_id: i32,
        prize_name: String,
        prize_note: String,
        prize_category: String,
        prize_amount: i32,
        companies_id: i32,
        percentage:i32
    )->Self{
         SpinPrizesEntity{
            prize_id,
            prize_name,
            prize_note,
            prize_category,
            prize_amount,
            companies_id,
            percentage
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
        percentage:i32
        

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

