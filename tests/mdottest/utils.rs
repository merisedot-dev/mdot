use std::str::FromStr;

/// Utility function to supplement [tokio::fs::File] reading.
pub fn parseu8(src: Vec<u8>) -> String {
    String::from_utf8(src).unwrap().trim_end().to_string().replace("    ", "\t")
}

/// Utiltiy function to avoid writing that match multiple times.
pub fn str2i8(value: String) -> i8 {
    match value.as_str() {
        "n" => -1,
        _ => value.as_str().parse::<i8>().unwrap(),
    }
}

#[derive(Debug, Default)]
pub enum MyBool {
    TRUE,
    #[default]
    FALSE,
}

impl FromStr for MyBool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "true" => Self::TRUE,
            "false" => Self::FALSE,
            _ => return Err("WTF".to_string()),
        })
    }
}

impl Into<bool> for MyBool {
    fn into(self) -> bool {
        match self {
            Self::TRUE => true,
            Self::FALSE => false,
        }
    }
}
