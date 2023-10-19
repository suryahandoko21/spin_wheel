use super::content_payload::ContentPayload;
use super::content_presenter::ContentPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::content_entity::ContentEntity;
pub struct ContentPresenterMapper {}

impl ApiMapper<ContentEntity, ContentPresenter, ContentPayload> for ContentPresenterMapper {
    fn to_api(entity: ContentEntity) -> ContentPresenter {
        ContentPresenter {
            id: entity.id,
            title: entity.content_title,
            description: entity.content_description,
        }
    }

    fn to_entity(_payload: ContentPayload) -> ContentEntity {
        panic!("not implemented");
    }
}
