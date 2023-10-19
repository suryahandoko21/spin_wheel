use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_company_entity::SpinCompanyEntity;
use super::models::Companies;
pub struct SpinCompaniesDbMapper {}
impl DBMapper<SpinCompanyEntity, Companies> for SpinCompaniesDbMapper {
    fn to_db(entity: SpinCompanyEntity) -> Companies {
        Companies {
            id:entity.id,
            uuid: entity.uuid,
            companies_code: entity.companies_code,
            companies_name: entity.companies_name,
            companies_address: entity.companies_address,
            is_company_enabled : entity.is_company_enabled,
            max_credit: entity.max_credit,
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
            companies_address:model.companies_address,
            is_company_enabled:model.is_company_enabled,
            max_credit:model.max_credit,
        }
    }
}
