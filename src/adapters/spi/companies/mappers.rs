use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_company_entity::SpinCompanyEntity;
use crate::domain::spin_promos_entity::SpinPromosEntity;

use super::models::Companies;


pub struct SpinCompaniesDbMapper {}


impl DBMapper<SpinCompanyEntity, Companies> for SpinCompaniesDbMapper {
    fn to_db(entity: SpinCompanyEntity) -> Companies {
        Companies {
            id:entity.id,
            uuid: entity.uuid,
            companies_code: entity.companies_code,
            companies_name: entity.companies_name,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
            
        }
    }

    fn to_entity(model: Companies) -> SpinCompanyEntity {
        SpinCompanyEntity {
            
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
            id: model.id,
            uuid: model.uuid,
            companies_code:model.companies_code,
            companies_name: model.companies_name,
        }
    }
}
