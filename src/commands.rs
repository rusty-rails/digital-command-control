use crate::power_management::PowerManagement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Command {
    PowerManagement(PowerManagement),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power_management::PowerOff;

    #[test]
    fn serialize() {
        let power_off = Command::PowerManagement(PowerManagement::PowerOff(PowerOff::MAIN));
        let stringified = serde_json::to_value(&power_off).unwrap();
        assert_eq!("<0 MAIN>", stringified.as_str().unwrap());
        let deserialized: Command = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off);
    }
}
