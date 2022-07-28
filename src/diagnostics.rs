// https://dcc-ex.com/reference/software/diagnostic-d-command.html#diagnostics-d-command
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Other {
    #[serde(rename = "<D CABS>")]
    Cabs,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Diagnostics {
    Other(Other),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_cabs() {
        let cabs = Diagnostics::Other(Other::Cabs);
        let stringified = serde_json::to_value(&cabs).unwrap();
        assert_eq!("<D CABS>", stringified.as_str().unwrap());
        let deserialized: Diagnostics = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, cabs);
    }
}
