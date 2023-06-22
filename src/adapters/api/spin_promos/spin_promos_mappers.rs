use crate::adapters::{api::spin_promos::spin_promos_presenters::SpinPromosPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_promos_entity::SpinPromosEntity;

use super::spin_promos_payloads::SpinPromosPayload;

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

    fn to_entity(_payload: SpinPromosPayload) -> SpinPromosEntity {
        // SpinPrizesEntity { 
        //     // prize_id: (), 
        //     // prize_weight: (),
        //     // prize_name: (), 
        //     // prize_note: (),
        //     // prize_category: (), 
        //     // prize_amount: () 
        // }
        panic!("not implemented");
    }
}
