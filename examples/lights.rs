use digital_command_control::cab::{Cab, CabFunction, Throttle};
use digital_command_control::diagnostics::Other;
use digital_command_control::power_management::PowerOn;
use std::str;
use std::time::Duration;
fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let builder = serialport::new(port_name, baud_rate);

    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        ::std::process::exit(1);
    });

    let mut clone = port.try_clone().expect("Failed to clone");

    let light_on = CabFunction {
        cab: 5,
        func: 0,
        state: true,
    };

    let light_off = CabFunction {
        cab: 5,
        func: 0,
        state: false,
    };

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(20000));
        clone
            .write_all(
                serde_json::to_value(&PowerOn::JOIN)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(&Other::Cabs)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(&Other::Cabs)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::Throttle(Throttle {
                    cab: 5,
                    speed: 0,
                    forward: true,
                }))
                .unwrap()
                .as_str()
                .unwrap()
                .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::CabFunction(light_on.clone()))
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::CabFunction(light_off.clone()))
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::Throttle(Throttle {
                    cab: 5,
                    speed: 0,
                    forward: false,
                }))
                .unwrap()
                .as_str()
                .unwrap()
                .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::CabFunction(light_on))
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        clone
            .write_all(
                serde_json::to_value(Cab::CabFunction(light_off))
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    });

    let mut buffer: [u8; 1] = [0; 1];
    loop {
        match port.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    print!("{}", str::from_utf8(&buffer).unwrap());
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
