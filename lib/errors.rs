use std::result::Result as StdResult;

/// Custom result type to encapsulate everything that happens in here.
pub type StagResult<T, E = StagError> = StdResult<T, E>;

/// Error coalescence enumeration, meant to be used with [StagResult].
pub enum StagError {
    // TODO
}
