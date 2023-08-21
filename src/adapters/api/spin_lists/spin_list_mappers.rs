use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_lists_entity::SpinListsPrizesEntity;

use super::spin_list_payloads::SpinListPayload;
use super::spin_list_presenters::SpinListsPrizesPresenter;

pub struct SpinListPrizesPresenterMapper {}

impl ApiMapper<SpinListsPrizesEntity, SpinListsPrizesPresenter, SpinListPayload> for SpinListPrizesPresenterMapper {
    fn to_api(entity: SpinListsPrizesEntity)->SpinListsPrizesPresenter {
       SpinListsPrizesPresenter { 
            spin_prizes: entity.spin_prizes,
            company_code: entity.company_code, 
            list_status: entity.list_status, 
            percentage: entity.percentage, 
            roleid : entity.roleid,
            created_at: entity.created_at, 
            created_by: entity.created_by, 
            updated_at: entity.updated_at, 
            updated_by: entity.updated_by 
    }
    }

    fn to_entity(payload:SpinListPayload)->SpinListsPrizesEntity {
        todo!()
    }
    
}
