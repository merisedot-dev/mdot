use stag::script::keys::Association;

/// Utility function to turn a simple string into a matchable [Association].
/// This is NOT meant to replace the real flow of conversion.
pub fn str2assoc(value: String) -> Association {
    match value.as_str() {
        "one2many" => Association::ONE2MANY(String::new()),
        "one2one" => Association::ONE2ONE(String::new()),
        "many2many" => Association::MANY2MANY,
        _ => Association::NONE, // failsafe
    }
}

