use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::content_entity::ContentEntity;

use super::models::Content;

pub struct ContentDbMapper {}
impl DBMapper<ContentEntity, Content> for ContentDbMapper {
    fn to_db(entity: ContentEntity) -> Content {
        Content {
            id: entity.id,
            companies_code: entity.companies_code,
            content_title: entity.content_title,
            content_description: entity.content_description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
            default: entity.default,
        }
    }

    fn to_entity(model: Content) -> ContentEntity {
        ContentEntity {
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
            id: model.id,
            companies_code: model.companies_code,
            content_title: model.content_title,
            content_description: model.content_description,
            default: model.default,
        }
    }
}
