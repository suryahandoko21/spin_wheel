use crate::adapters::spi::prizes::models::SpinPrizes;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_prizes_entity::SpinPrizesEntity;

pub struct SpinPrizesDbMapper {}

impl DBMapper<SpinPrizesEntity, SpinPrizes> for SpinPrizesDbMapper {
    fn to_db(entity: SpinPrizesEntity) -> SpinPrizes {
        SpinPrizes {
            id:entity.prize_id,
            prize_amount:entity.prize_amount,
            prize_category:entity.prize_category,
            prize_name:entity.prize_name,
            prize_note:entity.prize_note,
            prize_weight:entity.prize_weight
          
        }
    }

    fn to_entity(model: SpinPrizes) -> SpinPrizesEntity {
        SpinPrizesEntity {
            prize_id:model.id,
            prize_amount:model.prize_amount,
            prize_category:model.prize_category,
            prize_name:model.prize_name,
            prize_note:model.prize_note,
            prize_weight:model.prize_weight
        }
    }
}
