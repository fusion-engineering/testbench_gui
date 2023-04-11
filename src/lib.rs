pub fn generate_sequence(max_value: u128, time_per_step: u128, step_size: u128) -> Vec<[u128; 2]> {
    let mut time = 0;
    let mut dshot_value = 0;
    let n_steps = max_value / step_size;
    let mut sequence: Vec<[u128; 2]> = Vec::new();
    println!("{n_steps}");
    for i in 0..(n_steps * 2 + 1) {
        println!("i = {i},dshot = {dshot_value}");
        if i < n_steps {
            time += time_per_step;
            sequence.push([time, dshot_value]);
            dshot_value += step_size;
        } else if (i >= n_steps) && (dshot_value > 0) {
            time += time_per_step;
            sequence.push([time, dshot_value]);
            dshot_value -= step_size;
        }
    }
    time += time_per_step;
    sequence.push([time, dshot_value]);
    sequence
}

use byteorder::{BigEndian, ByteOrder};
use std::io;
use std::time::Duration;
pub struct Port {
    pub port: Box<dyn serialport::SerialPort>,
}
impl Port {
    pub fn open(portname: &str) -> Result<Port, serialport::Error> {
        let port_name = portname;
        match serialport::open(&port_name) {
            Ok(port) => {
                let mut new_port = port;
                new_port.set_timeout(Duration::from_millis(2000)).unwrap();
                new_port.set_baud_rate(9600).unwrap();
                Ok(Port { port: new_port })
            }
            Err(e) => Err(e),
        }
        // let mut new_port = serialport::open(&port_name).unwrap_or_else(|e| {
        //     eprintln!("Failed to open \"{port_name}\". Error: {e}");
        //     std::process::exit(1);
        // });
        // new_port.set_timeout(Duration::from_millis(2000)).unwrap();
        // new_port.set_baud_rate(9600).unwrap();
        // Ok(Port { port: new_port })
    }
    // pub fn open(portname: &str) -> Port {
    //     let port_name = portname;
    //     let mut new_port = serialport::open(&port_name).unwrap_or_else(|e| {
    //         eprintln!("Failed to open \"{port_name}\". Error: {e}");
    //         std::process::exit(1);
    //     });
    //     new_port.set_timeout(Duration::from_millis(2000)).unwrap();
    //     new_port.set_baud_rate(9600).unwrap();
    //     Port { port: new_port }
    // }
    pub fn clear_buffers(&mut self) {
        self.port
            .clear(serialport::ClearBuffer::Input)
            .expect("Failed to clear input buffer");
        self.port
            .clear(serialport::ClearBuffer::Output)
            .expect("Failed to discard output buffer");
    }

    // pub fn read_serial_data(&mut self) -> &str{
    //     let mut input_buffer = ArrayString::<[_; 64]>::new();
    //     while self.port.bytes_to_read().unwrap() < 8 {
    //         // wait until buffer is filled;
    //     }
    //     let mut data = "";
    //     loop {
    //         let mut buf = [0];
    //         match self.port.read(&mut buf) {
    //             Ok(1) => input_buffer.push(buf[0] as char),
    //             _ => {},
    //         }
    //         if let Some(input) = input_buffer.strip_suffix('\n') {
    //             data = input;
    //             input_buffer.clear();
    //             break
    //         }
    //     }
    //     data
    // }

    pub fn read_serial_data_raw(&mut self) -> std::vec::Vec<u8> {
        let mut read_buf = vec![0; 8];
        while self.port.bytes_to_read().unwrap() == 0 {
            // wait until buffer is filled;
            println!("waiting for incoming data")
        }
        match self.port.read(read_buf.as_mut_slice()) {
            Ok(_count) => {}
            _ => panic!("could not read"),
        }
        read_buf
    }

    pub fn read_serial_data_bytes(&mut self) -> [f32; 4] {
        let mut read_buf = vec![0; 14];
        while self.port.bytes_to_read().unwrap() < 14 {
            // wait until buffer is filled;
        }
        match self.port.read(&mut read_buf) {
            Ok(_count) => {}
            _ => panic!("could not read"),
        }
        assert!(read_buf.len() == 14);
        let thrust_raw: f32 = BigEndian::read_u32(&read_buf[0..4]) as f32;
        let torque_raw: f32 = BigEndian::read_u32(&read_buf[4..8]) as f32;
        let feedback: f32 = BigEndian::read_u32(&read_buf[8..12]) as f32;
        let current_raw: f32 = BigEndian::read_u16(&read_buf[12..]) as f32;
        [thrust_raw, torque_raw, feedback, current_raw]
    }

    pub fn start_measurement(&mut self) {
        // ask for user input. Send this user input to bluepill to start measuring
        //
        println!("Press enter to send 'start measurement' signal to arduino");
        let mut input_txt = String::new();
        let _string_to_send = b'a';
        io::stdin().read_line(&mut input_txt).unwrap();
        // self.port.write(&[string_to_send]).unwrap();
    }
}
