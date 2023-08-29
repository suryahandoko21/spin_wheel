use crate::adapters::api::spin_prizes::spin_prizes_presenters::SpinPrizesPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_reward_entity::SpinRewardEntity;
use super::spin_reward_payload::SpinRewardPayload;
use super::spin_reward_presenters::SpinRewardsPresenter;
pub struct SpinRewardPresenterMapper {}

impl ApiMapper<SpinRewardEntity, SpinRewardsPresenter,SpinRewardPayload> for SpinRewardPresenterMapper {
    fn to_api(entity: SpinRewardEntity) -> SpinRewardsPresenter {
        SpinRewardsPresenter {
            reward_id: entity.reward_id,
            reward_amount:entity.reward_amount, 
            reward_money:entity.reward_money,   
            reward_category:entity.reward_category,
            reward_name:entity.reward_name,
            reward_note:entity.reward_note,
            percentage:entity.percentage,
            reward_image: entity.reward_image,
            reward_status: entity.reward_status,
        }
    }

    fn to_entity(_payload: SpinRewardPayload) -> SpinRewardEntity {
        panic!("not implemented");
    }

}
