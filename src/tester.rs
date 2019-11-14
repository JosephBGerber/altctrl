// Binary file using a tcp communication protocol to the client

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::thread;

const PORT: &str = "/dev/AMA0";

pub fn main() {
    let stream_rx = serialport::open(PORT).unwrap();
    let mut stream_tx = stream_rx.try_clone().unwrap();

    thread::spawn(move || loop {
        let mut data = String::new();
        io::stdin().lock().read_line(&mut data).unwrap();
        stream_tx.write_all(data.as_bytes()).unwrap();
    });

    let mut buf_reader = BufReader::new(stream_rx);

    loop {
        let mut content = String::new();

        match buf_reader.read_line(&mut content) {
            Ok(_) => {
                print!("{}", content);
            }

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
