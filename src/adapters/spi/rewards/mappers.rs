
use crate::{application::mappers::db_mapper::DBMapper, domain::spin_reward_entity::SpinRewardEntity};
use super::models::SpinRewards;
pub struct SpinRewardsDbMapper {}

impl DBMapper<SpinRewardEntity, SpinRewards> for SpinRewardsDbMapper {
    fn to_db(entity: SpinRewardEntity) -> SpinRewards {
        SpinRewards {
            id: entity.reward_id,
            reward_name: entity.reward_name,
            reward_note: entity.reward_note,
            reward_category: entity.reward_category,
            reward_amount: entity.reward_amount,
            reward_money: entity.reward_money,
            reward_status: entity.reward_status,
            reward_order:entity.reward_order,
            companies_code: entity.companies_code,
            percentage: entity.percentage,
            reward_image: entity.reward_image,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    fn to_entity(model: SpinRewards) -> SpinRewardEntity {
        SpinRewardEntity {
            reward_id: model.id,
            reward_name: model.reward_name,
            reward_note: model.reward_note,
            reward_category: model.reward_category,
            reward_amount: model.reward_amount,
            reward_money: model.reward_money,
            reward_order:model.reward_order,
            companies_code: model.companies_code,
            percentage: model.percentage,
            reward_image: model.reward_image,
            reward_status: model.reward_status,
            created_at: model.created_at,
            updated_at: model.updated_at,
            // created_at :model.created_at,
            // updated_at:model.updated_at
        }
    }
}
