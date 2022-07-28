// https://dcc-ex.com/reference/software/command-summary.html#id1
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PowerOff {
    #[serde(rename = "<0 MAIN>")]
    MAIN,
    #[serde(rename = "<0 PROG>")]
    PROG,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PowerOn {
    #[serde(rename = "<1 MAIN>")]
    MAIN,
    #[serde(rename = "<1 PROG>")]
    PROG,
    #[serde(rename = "<1 JOIN>")]
    JOIN,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Other {
    #[serde(rename = "<c>")]
    Current,
    #[serde(rename = "<s>")]
    Status,
    #[serde(rename = "<D RESET>")]
    Reset,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PowerManagement {
    PowerOff(PowerOff),
    PowerOn(PowerOn),
    Other(Other),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_power_off() {
        let power_off_main = PowerManagement::PowerOff(PowerOff::MAIN);
        let stringified = serde_json::to_value(&power_off_main).unwrap();
        assert_eq!("<0 MAIN>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off_main);

        let power_off_prog = PowerManagement::PowerOff(PowerOff::PROG);
        let stringified = serde_json::to_value(&power_off_prog).unwrap();
        assert_eq!("<0 PROG>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off_prog);
    }

    #[test]
    fn serialize_power_on() {
        let power_off_main = PowerManagement::PowerOn(PowerOn::MAIN);
        let stringified = serde_json::to_value(&power_off_main).unwrap();
        assert_eq!("<1 MAIN>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off_main);

        let power_off_prog = PowerManagement::PowerOn(PowerOn::PROG);
        let stringified = serde_json::to_value(&power_off_prog).unwrap();
        assert_eq!("<1 PROG>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off_prog);
    }

    #[test]
    fn serialize_other() {
        let current = PowerManagement::Other(Other::Current);
        let stringified = serde_json::to_value(&current).unwrap();
        assert_eq!("<c>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, current);

        let status = PowerManagement::Other(Other::Status);
        let stringified = serde_json::to_value(&status).unwrap();
        assert_eq!("<s>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, status);

        let reset = PowerManagement::Other(Other::Reset);
        let stringified = serde_json::to_value(&reset).unwrap();
        assert_eq!("<D RESET>", stringified.as_str().unwrap());
        let deserialized: PowerManagement = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, reset);
    }
}
