
use crate::adapters::spi::spinlist::models::SpinLists;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_lists_entity::SpinListsEntity;

pub struct SpinListDbMapper {}
impl DBMapper<SpinListsEntity, SpinLists> for SpinListDbMapper {
    fn to_db(entity: SpinListsEntity) -> SpinLists {
        SpinLists {
            id:entity.list_id,
            company_code : entity.company_code,
            quantity : entity.quantity,
            list_status : entity.list_status,
            created_at : entity.created_at,
            updated_at : entity.updated_at,
            created_by : entity.created_by,
            updated_by : entity.updated_by,
            spin_prizes_id : entity.spin_prizes_id,
          
          
        }
    }

    fn to_entity(model: SpinLists) -> SpinListsEntity {
        SpinListsEntity {
            list_id: model.id,
            company_code : model.company_code,
            list_status : model.list_status,
            quantity : model.quantity,
            created_at : model.created_at, 
            updated_at : model.updated_at,
            created_by : model.created_by,
            updated_by : model.updated_by,
            spin_prizes_id : model.spin_prizes_id

        }
    }
}
