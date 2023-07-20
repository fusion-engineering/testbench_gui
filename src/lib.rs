use crate::containers::ComboBox;
use eframe::egui::*;
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
    // Code below is for Option <Port> rather than returning a Port
    // pub fn open(portname: &str) -> Result<Port, serialport::Error> {
    //     let port_name = portname;
    //     match serialport::open(&port_name) {
    //         Ok(port) => {
    //             let mut new_port = port;
    //             new_port.set_timeout(Duration::from_millis(2000)).unwrap();
    //             new_port.set_baud_rate(9600).unwrap();
    //             Ok(Port { port: new_port })
    //         }
    //         Err(e) => Err(e),
    //     }

    pub fn open(portname: &str) -> Port {
        let port_name = portname;
        let mut new_port = serialport::open(&port_name).unwrap_or_else(|e| {
            eprintln!("Failed to open \"{port_name}\". Error: {e}");
            std::process::exit(1);
        });
        new_port.set_timeout(Duration::from_millis(2000)).unwrap();
        new_port.set_baud_rate(9600).unwrap();
        Port { port: new_port }
    }

    pub fn clear_buffers(&mut self) {
        self.port
            .clear(serialport::ClearBuffer::Input)
            .expect("Failed to clear input buffer");
        self.port
            .clear(serialport::ClearBuffer::Output)
            .expect("Failed to discard output buffer");
    }

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

pub fn max_value_combobox(ui: &mut Ui, mut max_value: u128) -> InnerResponse<Option<()>> {
    ComboBox::from_label("max value")
        .selected_text(format!("{:?}", max_value))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut max_value, 500, "500");
            ui.selectable_value(&mut max_value, 600, "600");
            ui.selectable_value(&mut max_value, 700, "700");
            ui.selectable_value(&mut max_value, 800, "800");
            ui.selectable_value(&mut max_value, 900, "900");
            ui.selectable_value(&mut max_value, 1000, "1000");
            ui.selectable_value(&mut max_value, 1100, "1100");
            ui.selectable_value(&mut max_value, 1200, "1200");
            ui.selectable_value(&mut max_value, 1300, "1300");
            ui.selectable_value(&mut max_value, 1400, "1400");
            ui.selectable_value(&mut max_value, 1500, "1500");
            ui.selectable_value(&mut max_value, 1600, "1600");
            ui.selectable_value(&mut max_value, 1700, "1700");
            ui.selectable_value(&mut max_value, 1800, "1800");
            ui.selectable_value(&mut max_value, 1900, "1900");
            ui.selectable_value(&mut max_value, 2000, "2000");
        })
}

pub fn step_size_combobox(ui: &mut Ui, mut step_size: u128) -> InnerResponse<Option<()>> {
    ComboBox::from_label("step size")
        .selected_text(format!("{:?}", step_size))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut step_size, 100, "100");
            ui.selectable_value(&mut step_size, 200, "200");
            ui.selectable_value(&mut step_size, 300, "300");
            ui.selectable_value(&mut step_size, 400, "400");
            ui.selectable_value(&mut step_size, 500, "500");
        })
}
