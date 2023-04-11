use arrayvec::ArrayString;
use eframe::egui;
use std::fmt::Write as fmt_Write;
use std::fs;
// use std::io;
use std::io::Write;
use std::process::Command;
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
    bluepill: Option<Port>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            max_value: 500,
            time_per_step: 2000,
            step_size: 100,
            port_name: "/dev/tty.usbmodem11".to_owned(),
            show_name: false,
            dshot_sequence: Vec::new(),
            gen_seq: false,
            filename: "data".to_string(),
            log_text: "".to_string(),
            ports_available: "".to_string(),
            usb_connected: false,
            bluepill: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.heading("USB port");
            ui.separator();
            if ui.button("scan USB ports").clicked() {
                self.ports_available = "".to_string();
                let list = serialport::available_ports().unwrap();
                for port in list {
                    Command::new("echo")
                        .arg(format!("{:?}", port.port_name))
                        .spawn()
                        .expect("ls command failed to start");
                    self.ports_available.push_str(&port.port_name);
                }
            };
            ui.label(&self.ports_available);
            ui.separator();
            ui.horizontal(|ui| {
                let name_label = ui.label("Port name: ");
                ui.text_edit_singleline(&mut self.port_name)
                    .labelled_by(name_label.id);
            });

            if self.show_name {
                ui.label(format!("Connecting to port '{}' ", self.port_name));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Testbench Interface");
            ui.separator();
            ui.heading("Dshot Sequence Parameters");
            egui::ComboBox::from_label("max value")
                .selected_text(format!("{:?}", self.max_value))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.max_value, 500, "500");
                    ui.selectable_value(&mut self.max_value, 600, "600");
                    ui.selectable_value(&mut self.max_value, 700, "700");
                    ui.selectable_value(&mut self.max_value, 800, "800");
                    ui.selectable_value(&mut self.max_value, 900, "900");
                    ui.selectable_value(&mut self.max_value, 1000, "1000");
                    ui.selectable_value(&mut self.max_value, 1100, "1100");
                    ui.selectable_value(&mut self.max_value, 1200, "1200");
                    ui.selectable_value(&mut self.max_value, 1300, "1300");
                    ui.selectable_value(&mut self.max_value, 1400, "1400");
                    ui.selectable_value(&mut self.max_value, 1500, "1500");
                    ui.selectable_value(&mut self.max_value, 1600, "1600");
                    ui.selectable_value(&mut self.max_value, 1700, "1700");
                    ui.selectable_value(&mut self.max_value, 1800, "1800");
                    ui.selectable_value(&mut self.max_value, 1900, "1900");
                    ui.selectable_value(&mut self.max_value, 2000, "2000");
                });
            egui::ComboBox::from_label("step size")
                .selected_text(format!("{:?}", self.step_size))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.step_size, 100, "100");
                    ui.selectable_value(&mut self.step_size, 200, "200");
                    ui.selectable_value(&mut self.step_size, 300, "300");
                    ui.selectable_value(&mut self.step_size, 100, "400");
                    ui.selectable_value(&mut self.step_size, 200, "500");
                });
            egui::ComboBox::from_label("time per step [ms]")
                .selected_text(format!("{:?}", self.time_per_step))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.time_per_step, 500, "300");
                    ui.selectable_value(&mut self.time_per_step, 500, "400");
                    ui.selectable_value(&mut self.time_per_step, 500, "500");
                    ui.selectable_value(&mut self.time_per_step, 1000, "1000");
                    ui.selectable_value(&mut self.time_per_step, 2000, "2000");
                });

            if ui.button("Generate sequence").clicked() {
                self.dshot_sequence =
                    generate_sequence(self.max_value, self.time_per_step, self.step_size);
                self.gen_seq = true;
            };
            ui.separator();
            if ui.button("connect usb").clicked() {
                let mut bluepill = Port::open(&self.port_name);
                self.usb_connected = true;
            }
            // if ui.button("Connect to USB").clicked() {
            //     self.usb_connected = true;

            // match Some(Port::open(&self.port_name)) {
            //     Some(port) => self.bluepill = Some(SOmeport),
            //     None => {}
            // }
            // if let self.bluepill = Some(Port::open(&self.port_name)) {
            //     {}
            // } else {}
            // self.bluepill = Some(Port::open(&self.port_name));
            // };
            if self.usb_connected && ui.button("Start measurement").clicked() {
                if self.gen_seq {
                    let term = Arc::new(AtomicBool::new(false));
                    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();
                    println!("Testbench started");

                    // let mut bluepill = Port::open(&self.port_name);
                    let mut file = fs::File::create(&self.filename).expect("Error creating file");
                    let mut write_buf = ArrayString::<[_; 64]>::new();
                    let mut data_vector: std::vec::Vec<ArrayString<[_; 64]>> = Vec::new();
                    let mut i = 0;
                    let now = Instant::now();

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
                            let _ = self
                                .bluepill
                                .as_mut()
                                .map(|bluepill| bluepill.port.write(write_buf.as_bytes()));
                        } else if time > self.dshot_sequence[i][0] {
                            i += 1;
                        }

                        // Read back sensor data from bluepill
                        let mut input_buffer = ArrayString::<[_; 64]>::new();
                        let mut data_buffer = ArrayString::<[_; 64]>::new();
                        let mut buf: [u8; 1] = [0];
                        let mut read = true;

                        while read {
                            let _ = self
                                .bluepill
                                .as_mut()
                                .map(|bluepill| bluepill.port.read(&mut buf));
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
                    if let Some(bluepill) = self.bluepill.as_mut() {
                        bluepill.clear_buffers()
                    }
                    // self.bluepill
                    //     .as_mut()
                    //     .map(|bluepill| bluepill.clear_buffers());
                    println!("buffers cleared");
                } else if !self.gen_seq {
                    self.log_text = "please generate Dshot sequence first".to_string()
                }
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
