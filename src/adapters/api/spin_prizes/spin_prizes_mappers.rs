use crate::adapters::{api::spin_prizes::spin_prizes_payloads::SpinPrizesPayload,api::spin_prizes::spin_prizes_presenters::SpinPrizesPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_prizes_entity::SpinPrizesEntity;

pub struct SpinPrizesPresenterMapper {}

impl ApiMapper<SpinPrizesEntity, SpinPrizesPresenter, SpinPrizesPayload> for SpinPrizesPresenterMapper {
    fn to_api(entity: SpinPrizesEntity) -> SpinPrizesPresenter {
        SpinPrizesPresenter {
            prize_id: entity.prize_id,
            prize_amount:entity.prize_amount,    
            prize_category:entity.prize_category,
            prize_name:entity.prize_name,
            prize_note:entity.prize_note,
        }
    }

    fn to_entity(_payload: SpinPrizesPayload) -> SpinPrizesEntity {
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
