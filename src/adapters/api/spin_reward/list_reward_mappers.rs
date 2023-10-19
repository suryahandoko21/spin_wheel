use super::spin_reward_payload::SpinRewardPayload;
use super::spin_reward_presenters::ListRewardsPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::spin_reward_entity::SpinRewardEntity;
pub struct ListRewardPresenterMapper {}

impl ApiMapper<SpinRewardEntity, ListRewardsPresenter, SpinRewardPayload>
    for ListRewardPresenterMapper
{
    fn to_api(entity: SpinRewardEntity) -> ListRewardsPresenter {
        ListRewardsPresenter {
            rewardType: entity.reward_category,
            rewardName: entity.reward_name,
            rewardDescriptions: entity.reward_note,
            money: entity.reward_money,
        }
    }

    fn to_entity(_payload: SpinRewardPayload) -> SpinRewardEntity {
        panic!("not implemented");
    }
}
