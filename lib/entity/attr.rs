use crate::attribute::EntityAttribute;

#[derive(Clone)]
pub struct AttrValue<T>
where
    T: EntityAttribute,
{
    nullable: bool,
    attr_type: T,
}

impl<T> AttrValue<T>
where
    T: EntityAttribute,
{
    pub fn new(nullable: bool, attr_type: T) -> Self {
        Self {
            nullable,
            attr_type,
        }
    }
}
