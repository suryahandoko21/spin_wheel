use crate::adapters::spi::prizes::models::SpinPrizes;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_prizes_entity::{SpinPrizesEntity, SpinPrizesCompaniesEntity};

use super::models::SpinPrizesCompanies;

pub struct SpinPrizesDbMapper {}
pub struct SpinPrizesCompaniesDbMapper {}
impl DBMapper<SpinPrizesCompaniesEntity, SpinPrizesCompanies> for SpinPrizesCompaniesDbMapper {
    fn to_db(_entity:SpinPrizesCompaniesEntity)->SpinPrizesCompanies {
        todo!()
    }

    fn to_entity(model:SpinPrizesCompanies)->SpinPrizesCompaniesEntity {
        SpinPrizesCompaniesEntity{
            prize_name: model.prize_name,
            prize_note: model.prize_note,
            prize_category: model.prize_category,
            prize_amount: model.prize_amount,
            companies: model.companies,
            percentage:model.percentage
        }
    }
}
impl DBMapper<SpinPrizesEntity, SpinPrizes> for SpinPrizesDbMapper {
    fn to_db(entity: SpinPrizesEntity) -> SpinPrizes {
        SpinPrizes {
            id:entity.prize_id,
            prize_amount:entity.prize_amount,
            prize_category:entity.prize_category,
            prize_name:entity.prize_name,
            prize_note:entity.prize_note,
            companies_id: entity.companies_id,
            percentage:entity.percentage
          
          
        }
    }

    fn to_entity(model: SpinPrizes) -> SpinPrizesEntity {
        SpinPrizesEntity {
            prize_id:model.id,
            prize_amount:model.prize_amount,
            prize_category:model.prize_category,
            prize_name:model.prize_name,
            prize_note:model.prize_note,
            companies_id: model.companies_id,
            percentage:model.percentage
        }
    }
}
