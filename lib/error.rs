pub type StagResult<T, E = StagError> = Result<T, E>;

#[derive(Debug)]
pub enum StagError {}
