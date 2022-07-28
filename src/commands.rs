use crate::cab::Cab;
use crate::diagnostics::Diagnostics;
use crate::power_management::PowerManagement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Command {
    PowerManagement(PowerManagement),
    Cab(Cab),
    Diagnostics(Diagnostics),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cab::CabFunction;
    use crate::diagnostics::Diagnostics;
    use crate::power_management::PowerOff;

    #[test]
    fn serialize_power_off() {
        let power_off = Command::PowerManagement(PowerManagement::PowerOff(PowerOff::MAIN));
        let stringified = serde_json::to_value(&power_off).unwrap();
        assert_eq!("<0 MAIN>", stringified.as_str().unwrap());
        let deserialized: Command = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, power_off);
    }

    #[test]
    fn serialize_cab() {
        let cab_function = CabFunction {
            cab: 5,
            func: 1,
            state: true,
        };
        let cab_fun = Cab::CabFunction(cab_function);
        let stringified = serde_json::to_value(&cab_fun).unwrap();
        assert_eq!("<F 5 1 1>", stringified.as_str().unwrap());
        let deserialized: Cab = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, cab_fun);
    }

    #[test]
    fn serialize_diagnostics() {
        let cabs = Diagnostics::Other(crate::diagnostics::Other::Cabs);
        let stringified = serde_json::to_value(&cabs).unwrap();
        assert_eq!("<D CABS>", stringified.as_str().unwrap());
        let deserialized: Diagnostics = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, cabs);
    }
}
