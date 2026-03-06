use crate::constraint::SQLConstraint;

pub mod foreign;
pub mod unique;

pub trait SQLConverter<T>
where
    T: SQLConstraint,
{
    /// Turns the given item into an SQL-compliant script snippet.
    fn convert(&self, item: T) -> String;
}
