use crate::{attribute::EntityAttribute, entity::Entity};

pub struct GraphLink<T>
where
    T: EntityAttribute,
{
    entity: Entity<T>,
}
