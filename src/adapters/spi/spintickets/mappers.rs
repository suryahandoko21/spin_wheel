use super::models::SpinTickets;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_tickets_entity::SpinTicketsEntity;
pub struct SpinTicketDBMapper {}
impl DBMapper<SpinTicketsEntity, SpinTickets> for SpinTicketDBMapper {
    fn to_db(_entity: SpinTicketsEntity) -> SpinTickets {
        todo!()
    }

    fn to_entity(model: SpinTickets) -> SpinTicketsEntity {
        SpinTicketsEntity {
            id: model.id,
            user_uuid: model.user_uuid,
            userid: model.userid,
            username: model.username,
            ticket_id: model.ticket_id,
            ticket_uuid: model.ticket_uuid,
            status: model.status,
            pointrule_id: model.pointrule_id,
            expired_date: model.expired_date,
        }
    }
}
