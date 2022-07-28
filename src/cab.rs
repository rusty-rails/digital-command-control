// https://dcc-ex.com/reference/software/command-summary.html#id2
use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Throttle {
    pub cab: u32,
    pub speed: i32,
    pub forward: bool,
}

pub fn throttle_serialize<S>(throttle: &Throttle, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!(
        "<t 1 {} {} {}>",
        throttle.cab, throttle.speed, throttle.forward as i32
    ))
}

fn throttle_deserializer<'de, D>(deserializer: D) -> Result<Throttle, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    let values: Vec<&str> = s
        .trim_matches(|p| p == '<' || p == '>')
        .split(' ')
        .collect();
    match values[0] {
        "t" => {
            let throttle = Throttle {
                cab: values[2].parse::<u32>().unwrap(),
                speed: values[3].parse::<i32>().unwrap(),
                forward: match values[4].parse::<usize>().unwrap() {
                    0 => false,
                    1 => true,
                    _ => true,
                },
            };
            Ok(throttle)
        }
        _ => Err(de::Error::custom("no throttle")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CabFunction {
    pub cab: i32,
    pub func: i32,
    pub state: bool,
}

pub fn cab_function_serialize<S>(cab_function: &CabFunction, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!(
        "<F {} {} {}>",
        cab_function.cab, cab_function.func, cab_function.state as i32
    ))
}

fn cab_function_deserializer<'de, D>(deserializer: D) -> Result<CabFunction, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    let values: Vec<&str> = s
        .trim_matches(|p| p == '<' || p == '>')
        .split(' ')
        .collect();
    match values[0] {
        "F" => {
            let cab_function = CabFunction {
                cab: values[1].parse::<i32>().unwrap(),
                func: values[2].parse::<i32>().unwrap(),
                state: match values[3].parse::<usize>().unwrap() {
                    0 => false,
                    1 => true,
                    _ => true,
                },
            };
            Ok(cab_function)
        }
        _ => Err(de::Error::custom("no cab function")),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Other {
    #[serde(rename = "<!>")]
    Stop,
    #[serde(rename = "<#>")]
    Cabs,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Cab {
    Other(Other),
    #[serde(
        serialize_with = "cab_function_serialize",
        deserialize_with = "cab_function_deserializer"
    )]
    CabFunction(CabFunction),
    #[serde(
        serialize_with = "throttle_serialize",
        deserialize_with = "throttle_deserializer"
    )]
    Throttle(Throttle),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_cab_function() {
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
    fn serialize_throttle() {
        let throttle = Throttle {
            cab: 5,
            speed: 0,
            forward: true,
        };
        let throttle = Cab::Throttle(throttle);
        let stringified = serde_json::to_value(&throttle).unwrap();
        assert_eq!("<t 1 5 0 1>", stringified.as_str().unwrap());
        let deserialized: Cab = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, throttle);
    }

    #[test]
    fn serialize_other() {
        let stop = Cab::Other(Other::Stop);
        let stringified = serde_json::to_value(&stop).unwrap();
        assert_eq!("<!>", stringified.as_str().unwrap());
        let deserialized: Cab = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, stop);

        let cabs = Cab::Other(Other::Cabs);
        let stringified = serde_json::to_value(&cabs).unwrap();
        assert_eq!("<#>", stringified.as_str().unwrap());
        let deserialized: Cab = serde_json::from_str(&stringified.to_string()).unwrap();
        assert_eq!(deserialized, cabs);
    }
}
