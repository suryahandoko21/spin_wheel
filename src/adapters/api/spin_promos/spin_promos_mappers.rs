use crate::adapters::{api::spin_promos::spin_promos_presenters::SpinPromosPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_promos_entity::SpinPromosEntity;
use crate::helpers::fn_global::convert_str_to_timestamp;

use super::spin_promos_payloads::SpinPromosPayload;
use chrono::Local;
pub struct SpinPromosPresenterMapper {}
impl ApiMapper<SpinPromosEntity, SpinPromosPresenter, SpinPromosPayload> for SpinPromosPresenterMapper {
    fn to_api(entity: SpinPromosEntity) -> SpinPromosPresenter {
        SpinPromosPresenter {
            promo_amount: entity.promo_amount,
            promo_status: entity.promo_status,
            user_id: entity.user_id,
            username:entity.username,
            expired_at: entity.expired_at,
            point_currention_time: entity.point_currention_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    fn to_entity(payload: SpinPromosPayload) -> SpinPromosEntity {
        SpinPromosEntity{
            promo_amount: payload.promo_amount,
            promo_status: "unused".to_string(),
            user_id: payload.user_id,
            username: payload.username,
            point_currention_time: convert_str_to_timestamp(payload.point_currention_time.to_string()),
            expired_at: convert_str_to_timestamp(payload.expired_at.to_string()),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
            created_by: "system".to_string(),
            updated_by: "system".to_string(),
        }
      
    }
}
