use digital_command_control::cab::{Cab, Throttle};
use digital_command_control::power_management::{PowerManagement, PowerOn};
use digital_command_control::serialport::SerialPort;
use digital_command_control::Command;
use std::time::Duration;

fn main() {
    let mut port = SerialPort::default();
    port.connect();

    let forward = Command::Cab(Cab::Throttle(Throttle {
        cab: 5,
        speed: 10,
        forward: true,
    }));

    let backward = Command::Cab(Cab::Throttle(Throttle {
        cab: 5,
        speed: 10,
        forward: false,
    }));

    let stop = Command::Cab(Cab::Throttle(Throttle {
        cab: 5,
        speed: 0,
        forward: false,
    }));

    std::thread::sleep(Duration::from_millis(20000));
    port.send(&Command::PowerManagement(PowerManagement::PowerOn(
        PowerOn::JOIN,
    )))
    .unwrap();
    std::thread::sleep(Duration::from_millis(2000));
    port.send(&forward).unwrap();
    std::thread::sleep(Duration::from_millis(2000));
    port.send(&stop.clone()).unwrap();
    std::thread::sleep(Duration::from_millis(1000));
    port.send(&backward).unwrap();
    std::thread::sleep(Duration::from_millis(2000));
    port.send(&stop.clone()).unwrap();
    std::thread::sleep(Duration::from_millis(2000));
}
