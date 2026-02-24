use crate::{errors::StagResult, graph::Graph};

/// Main logic trait of our graph handling, it will take an MCD or MLD [Graph] to
/// output a SQL script. Specifics will depend of the target SGBD, aka the
/// trait implementor. In case of errors in the [Graph], the error will be
/// passed to the resulting script, and it's on the user to fix it.
///
/// **Note**: In case of an MCD graph, a first conversion will be performed to
/// ensure the conversion doesn't mess things up, since it only works for MLD
/// graphs.
pub trait ConversionCore {
    /// Performs the actual conversion from [Graph] to script. The resulting
    /// script will not be stored to ensure no undesirable side effect happens.
    fn convert(&self, graph: Graph) -> StagResult<impl ToString>;
}

// TODO re-exports
