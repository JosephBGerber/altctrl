// Binary file using a tcp communication protocol to the client

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

use serialport::prelude::*;

const PORT: &str = "/dev/ttyACM0";

pub fn main() {
    let s = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(1),
    };

    let stream_rx = serialport::open_with_settings(PORT, &s).unwrap();
    let mut stream_tx = stream_rx.try_clone().unwrap();

    thread::spawn(move || loop {
        let mut data = String::new();
        io::stdin().lock().read_line(&mut data).unwrap();

        match stream_tx.write_all(data.as_bytes()) {
            Ok(_) => {}

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

            Err(e) => {
                panic!("{:?}", e);
            }
        }
    });

    let mut buf_reader = BufReader::new(stream_rx);

    let mut content = String::new();

    loop {
        match buf_reader.read_line(&mut content) {
            Ok(_) => {
                print!("{}", content);
                content = String::new();
            }

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
