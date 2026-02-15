/// Generic trait to simplify how different SGBDs will handle attributes.
/// This will require specialisation for each and every SGBD we intend to support.
pub trait EntityAttribute {
    /// Return the attribute type signature as a [String], so it can be used to
    /// write scripts later down the line.
    fn signature(&self) -> String;
}
