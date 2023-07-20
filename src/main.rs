use arrayvec::ArrayString;
use eframe::egui;
use std::fmt::Write as fmt_Write;
use std::fs;
// use std::io;
use serialport::SerialPortType;

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use testbench_gui::*;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

pub struct MyApp {
    available_serial_ports: Option<Vec<serialport::SerialPortInfo>>,
    serial_port: String,
    max_value: u128,
    time_per_step: u128,
    step_size: u128,
    port_name: String,
    show_name: bool,
    dshot_sequence: Vec<[u128; 2]>,
    gen_seq: bool,
    filename: String,
    log_text: String,
    ports_available: String,
    usb_connected: bool,
    // bluepill: Option<Port>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            available_serial_ports: None,
            serial_port: "/dev/tty.usbmodem11".to_string(),
            max_value: 500,
            time_per_step: 2000,
            step_size: 100,
            port_name: "".to_owned(),
            show_name: false,
            dshot_sequence: Vec::new(),
            gen_seq: false,
            filename: "data".to_string(),
            log_text: "".to_string(),
            ports_available: "".to_string(),
            usb_connected: false,
            // bluepill: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.heading("USB port");
            ui.separator();
            egui::ComboBox::from_id_source("serial-port")
                .width(ui.available_width() - 10.0)
                .selected_text(&self.serial_port)
                .show_ui(ui, |ui| {
                    let ports = self
                        .available_serial_ports
                        .get_or_insert_with(|| serialport::available_ports().unwrap_or_default());
                    let mut last_type = "Other:";
                    for p in ports.iter() {
                        let (t, desc) = match &p.port_type {
                            SerialPortType::UsbPort(usb) => ("USB:", {
                                let mut desc = format!("USB ID: {:04x}:{:04x}", usb.vid, usb.pid);
                                if let Some(m) = &usb.manufacturer {
                                    desc += &format!("\nManufacturer: {}", m);
                                }
                                if let Some(p) = &usb.product {
                                    desc += &format!("\nProduct: {}", p);
                                }
                                if let Some(s) = &usb.serial_number {
                                    desc += &format!("\nSerial number: {}", s);
                                }
                                Some(desc)
                            }),
                            SerialPortType::BluetoothPort => ("Bluetooth:", None),
                            SerialPortType::PciPort => ("Built-in:", None),
                            _ => ("Other:", None),
                        };
                        if t != last_type {
                            ui.label(t);
                            last_type = t;
                        }
                        let r = ui.selectable_value(
                            &mut self.serial_port,
                            p.port_name.clone(),
                            format!("{}", p.port_name),
                        );
                        if let Some(desc) = desc {
                            r.on_hover_text(desc);
                        }
                    }
                    if ports.is_empty() {
                        ui.label("No serial ports detected.");
                    }
                });
            // Serial port name
            // self.serial_port = self.port_name.clone();
            ui.label(&self.ports_available);
            ui.separator();
            ui.horizontal(|ui| {
                let name_label = ui.label("Port name: ");
                ui.text_edit_singleline(&mut self.serial_port)
                    .labelled_by(name_label.id);
            });

            // if ui.button("connect usb").clicked() {
            // self.log_text = "".to_string();
            // let bluepill = Port::open(&self.serial_port);
            // match bluepill {
            //     Ok(_port) => {
            //         self.usb_connected = true;
            //         self.log_text =
            //             format!("Succesfully connected to {:?}", self.port_name).to_string()
            //     }
            //     Err(e) => self.log_text = format!("could not open port due to {e}").to_string(),
            // }
            // }
            if self.show_name {
                ui.label(format!("Connecting to port '{}' ", &self.serial_port));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Testbench Interface");
            ui.separator();
            ui.heading("Dshot Sequence Parameters");
            egui::ComboBox::from_label("max value")
                .selected_text(format!("{:?}", self.max_value))
                .width(ui.available_width() / 2.0)
                .show_ui(ui, |ui| {
                    for dshot in [
                        500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700,
                        1800, 1900, 2000,
                    ] {
                        ui.selectable_value(&mut self.max_value, dshot, dshot.to_string());
                    }
                });
            egui::ComboBox::from_label("step size")
                .selected_text(format!("{:?}", self.step_size))
                .width(ui.available_width() / 2.0)
                .show_ui(ui, |ui| {
                    for step_size in [100, 200, 300, 400, 500] {
                        ui.selectable_value(&mut self.step_size, step_size, step_size.to_string());
                    }
                });

            egui::ComboBox::from_label("time per step [ms]")
                .selected_text(format!("{:?}", self.time_per_step))
                .width(ui.available_width() / 2.0)
                .show_ui(ui, |ui| {
                    for time in [100, 200, 500, 1000, 2000] {
                        ui.selectable_value(&mut self.time_per_step, time, time.to_string());
                    }
                });

            if ui.button("Generate sequence").clicked() {
                self.dshot_sequence =
                    generate_sequence(self.max_value, self.time_per_step, self.step_size);
                self.gen_seq = true;
            };
            ui.separator();

            if ui.button("Start measurement").clicked() {
                // ctrl-c handling (NOT WORKING?)
                let term = Arc::new(AtomicBool::new(false));
                signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();

                println!("Testbench started");
                let mut file = fs::File::create(&self.filename).expect("Error creating file");
                let mut write_buf = ArrayString::<[_; 64]>::new();
                let mut data_vector: std::vec::Vec<ArrayString<[_; 64]>> = Vec::new();
                let mut i = 0;
                let now = Instant::now();

                //generate sequence
                // if self.gen_seq {
                self.log_text = "".to_string();
                let mut bluepill = Port::open(&self.serial_port);
                // } else if !self.gen_seq {
                //     self.log_text = "please generate Dshot sequence first".to_string()
                // }

                // loop over sequence
                while (!term.load(Ordering::Relaxed))
                    && (now.elapsed().as_millis())
                        < self.dshot_sequence[self.dshot_sequence.len() - 1][0]
                {
                    // Write throttle command to bluepill
                    let time = now.elapsed().as_millis();
                    write_buf = ArrayString::<[_; 64]>::new();
                    if time <= self.dshot_sequence[i][0] {
                        writeln!(write_buf, "A{}", self.dshot_sequence[i][1]).unwrap();
                        let _ = bluepill.port.write(write_buf.as_bytes());
                        // println!("{write_buf}")
                    } else if time > self.dshot_sequence[i][0] {
                        i += 1;
                    }

                    // Read back sensor data from bluepill
                    let mut input_buffer = ArrayString::<[_; 64]>::new();
                    let mut data_buffer = ArrayString::<[_; 64]>::new();
                    let mut buf = [0];
                    let mut read = true;
                    while read {
                        let _ = bluepill.port.read(&mut buf);
                        input_buffer.push(buf[0] as char);
                        if let Some(data) = input_buffer.strip_suffix('\n') {
                            data_buffer.push_str(&format!("{},", &time.to_string()));
                            data_buffer.push_str(data);
                            read = false;
                        }
                    }
                    println!("{data_buffer}");
                    data_vector.push(data_buffer);
                }

                // Exit procedure: write data into file, send 0 to motor and clear buffer
                for i in &data_vector {
                    writeln!(file, "{i}").expect("Could not write file");
                }
                writeln!(write_buf, "A{}", 000).unwrap();
                bluepill.clear_buffers();
                println!("buffers cleared");
            }
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Sequence Display");
            ui.separator();
            if self.gen_seq {
                ui.label(format!("{:?}", self.dshot_sequence));
            }
        });
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            ui.heading("Info Text");
            ui.separator();
            ui.label(&self.log_text);
        });
    }
}
