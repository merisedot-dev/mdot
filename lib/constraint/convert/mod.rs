pub mod foreign;
pub mod unique;

use crate::{constraint::SQLConstraint, errors::StagResult};

/// Encapsulation trait to turn a set [SQLConstraint] into an SQL-compliant
/// script snippet. In case of a malformed constraint, it should output a
/// malformed SQL script.
pub trait SQLConverter<T>
where
    T: SQLConstraint,
{
    /// Turns the given [SQLConstraint] into an SQL-compliant script snippet.
    fn convert(&self, item: T) -> StagResult<String>;
}
