use crate::Command;
pub struct SerialPort {
    pub name: String,
    pub baud_rate: u32,
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl Default for SerialPort {
    fn default() -> Self {
        SerialPort {
            name: "/dev/ttyACM0".to_string(),
            baud_rate: 115200,
            port: None,
        }
    }
}

impl SerialPort {
    pub fn connect(&mut self) {
        let builder = serialport::new(&self.name, self.baud_rate);

        let port = builder.open().unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", self.name, e);
            ::std::process::exit(1);
        });
        self.port = Some(port);
    }

    pub fn send(&mut self, command: &Command) -> std::io::Result<()> {
        self.port.as_deref_mut().unwrap().write_all(
            serde_json::to_value(&command)
                .unwrap()
                .as_str()
                .unwrap()
                .as_bytes(),
        )?;
        Ok(())
    }
}
