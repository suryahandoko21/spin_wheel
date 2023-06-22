pub trait DBMapper<Entity,DbModel>{
    fn to_db(entity:Entity)->DbModel;

    fn to_entity(model:DbModel)->Entity;
}

pub trait DBMapperDb<Entity,DbModel>{
    // fn to_db(entity:Entity)->DbModel;

    fn to_entity(model:DbModel)->Entity;
}

