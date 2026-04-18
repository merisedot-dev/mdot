use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
};

use gettextrs::gettext;

#[derive(Debug)]
pub enum ProjectError {
    // common errors
    MISSINGNAME,
    MISSINGPATH,
    MISSINGCORE,
    // more... Exotic errors
    UNACCESSIBLEPATH(PathBuf),
}

// ready-to-use error messages
impl Display for ProjectError {
    fn fmt(&self, frm: &mut Formatter<'_>) -> FmtResult {
        frm.write_str(
            match self {
                // common error messages
                Self::MISSINGNAME => gettext("__MissingProjName"),
                Self::MISSINGPATH => gettext("__MissingProjPath"),
                Self::MISSINGCORE => gettext("__MissingProjCore"),
                // more complex error messages
                Self::UNACCESSIBLEPATH(path) => {
                    format!(
                        "{}: {}",
                        gettext("__UnaccessibleProjPath"),
                        path.to_str().unwrap_or_default()
                    )
                }
            }
            .as_str(),
        )
    }
}
