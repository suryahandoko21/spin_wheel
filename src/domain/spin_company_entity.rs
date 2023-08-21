use std::time::SystemTime;

#[derive(Debug)]
pub struct SpinCompanyEntity{
    pub id : i32,
    pub uuid : String,
    pub companies_code : String,
    pub companies_name:String,
    pub created_at: SystemTime,
    pub created_by: String,
    pub updated_at: SystemTime,
    pub updated_by :String,
   

}
impl SpinCompanyEntity {
    pub fn new(
        id : i32,
        uuid: String,
        companies_code: String,
        companies_name:String,
        created_at : SystemTime, 
        updated_at : SystemTime,
        created_by : String,
        updated_by : String,
  
    )->Self{
        SpinCompanyEntity{
            id,
            uuid,
            companies_code,
            companies_name,
            created_at,
            created_by,
            updated_at,
            updated_by,
        }

    }
}