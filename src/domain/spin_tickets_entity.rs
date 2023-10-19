#[derive(Debug, Clone)]
pub struct SpinTicketsEntity {
    pub id: i32,
    pub user_uuid: String,
    pub userid: String,
    pub username: String,
    pub ticket_id: i32,
    pub ticket_uuid: String,
    pub status: String,
    pub pointrule_id: i32,
    pub expired_date: String,
}

impl SpinTicketsEntity {
    pub fn new(
        id: i32,
        user_uuid: String,
        userid: String,
        username: String,
        ticket_id: i32,
        ticket_uuid: String,
        status: String,
        pointrule_id: i32,
        expired_date: String,
    ) -> Self {
        SpinTicketsEntity {
            id,
            user_uuid,
            userid,
            username,
            ticket_id,
            ticket_uuid,
            status,
            pointrule_id,
            expired_date,
        }
    }
}
