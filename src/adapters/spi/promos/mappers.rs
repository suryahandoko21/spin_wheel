
use crate::adapters::spi::promos::models::SpinPromos;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_promos_entity::SpinPromosEntity;
pub struct SpinPromosDbMapper {}

impl DBMapper<SpinPromosEntity, SpinPromos> for SpinPromosDbMapper {
    fn to_db(entity: SpinPromosEntity) -> SpinPromos {
        SpinPromos {
            id: entity.promo_id,
            promo_amount: entity.promo_amount,
            promo_status : entity.promo_status,
            user_id:entity.user_id,
            username:entity.username, 
            point_currention_time: entity.point_currention_time,
            expired_at: entity.expired_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    fn to_entity(model: SpinPromos) -> SpinPromosEntity {
        SpinPromosEntity {
            promo_id: model.id,
            promo_amount: model.promo_amount,
            promo_status: model.promo_status,
            user_id: model.user_id,
            username:model.username,
            point_currention_time: model.point_currention_time,
            expired_at: model.expired_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }
}
