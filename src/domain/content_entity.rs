use std::time::SystemTime;
#[derive(Debug)]
pub struct ContentEntity {
    pub id: i32,
    pub companies_code: String,
    pub content_title: String,
    pub content_description: String,
    pub created_at: SystemTime,
    pub created_by: String,
    pub updated_at: SystemTime,
    pub updated_by: String,
    pub default: bool,
}
impl ContentEntity {
    pub fn new(
        id: i32,
        companies_code: String,
        content_title: String,
        content_description: String,
        created_at: SystemTime,
        updated_at: SystemTime,
        created_by: String,
        updated_by: String,
        default: bool,
    ) -> Self {
        ContentEntity {
            id,
            companies_code,
            content_title,
            content_description,
            created_at,
            created_by,
            updated_at,
            updated_by,
            default,
        }
    }
}
