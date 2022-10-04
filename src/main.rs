#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::{self, TextStyle, FontFamily::Proportional}, epaint::FontId};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Spacing Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    input: String,
    temp_name: String,
    temp_size: String,
    spacers: Vec<Spacer>,
    prev_outputs: Vec<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Body, FontId::new(20.0, Proportional)),
            (TextStyle::Button, FontId::new(20.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
        if self.spacers.is_empty() {
            self.spacers.push(Spacer { name: "1/2 Nylon".to_string(), size: 0.5, used: true });
            self.spacers.push(Spacer { name: "3/8 Nylon".to_string(), size: 0.375, used: true });
            self.spacers.push(Spacer { name: "1/4 Nylon".to_string(), size: 0.25, used: true });
            self.spacers.push(Spacer { name: "1/8 Nylon".to_string(), size: 0.125, used: true });
            self.spacers.push(Spacer { name: "Thick Teflon".to_string(), size: 0.0625, used: true });
            self.spacers.push(Spacer { name: "Black Steel".to_string(), size: 0.03125, used: true });
            self.spacers.push(Spacer { name: "Thin Teflon".to_string(), size: 0.03125, used: true });
            self.spacers.push(Spacer { name: "Small Black".to_string(), size: 0.181, used: true });
            self.spacers.push(Spacer { name: "Large Black".to_string(), size: 0.315, used: true });
            self.spacers.push(Spacer { name: "Thick Steel".to_string(), size: 0.0392, used: true });
            self.spacers.push(Spacer { name: "Thin Steel".to_string(), size: 0.0215, used: true });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.spacers.sort_by(|a, b| b.size.total_cmp(&a.size));
            ui.add(egui::TextEdit::singleline(&mut self.input).hint_text("Spacing"));
            if !self.input.is_empty() && self.input.parse::<f32>().is_ok() {
                let mut used_input: f32 = 0.0;
                let input_num = self.input.parse::<f32>().unwrap();
                let mut sinputs: Vec<f32> = vec![];
                for _i in &mut self.spacers {
                    sinputs.push(0.0);
                }
                for i in 0..self.spacers.len() {
                    while self.spacers[i].size <= input_num - used_input && self.spacers[i].used {
                        sinputs[i] += 1.0;
                        used_input += self.spacers[i].size;
                    }
                }
                ui.label("Off by ".to_string() + &(input_num - used_input).to_string());
                for i in 0..self.spacers.len() {
                    if sinputs[i] > 0.0 {
                        ui.label(sinputs[i].to_string() + " " + &self.spacers[i].name.to_string() + " " + &self.spacers[i].size.to_string());
                    }
                }
                if ui.button("Save Output").is_pointer_button_down_on() {
                    let mut spacing_output: String = self.input.to_string() + " in";
                    spacing_output += &("\n\tOff by ".to_string() + &(input_num - used_input).to_string());
                    for i in 0..self.spacers.len() {
                        if sinputs[i] > 0.0 {
                            spacing_output += &("\n\t".to_string() + &sinputs[i].to_string() + " " + &self.spacers[i].name.to_string() + " " + &self.spacers[i].size.to_string());
                        }
                    }
                    self.prev_outputs.push(spacing_output);
                    self.input = Default::default();
                }
            }
            if !self.prev_outputs.is_empty() {
                ui.collapsing("Previous Outputs", |ui| {
                    let mut prev_output_str: String = Default::default();
                    for i in 0..self.prev_outputs.len() - 1 {
                        prev_output_str += &(self.prev_outputs[i].to_string() + &"\n".to_string());
                    }
                    prev_output_str += &self.prev_outputs[self.prev_outputs.len() - 1].to_string();
                    ui.label(prev_output_str);
                });
            }
            ui.separator();
            ui.collapsing("Spacers", |ui| {
                for i in &mut self.spacers {
                    ui.checkbox(&mut i.used, i.size.to_string() + " " + &i.name.to_string());
                }
            });
            ui.collapsing("Add Spacer", |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.temp_name).hint_text("Name"));
                ui.add(egui::TextEdit::singleline(&mut self.temp_size).hint_text("Size"));
                if ui.button("Add Spacer").is_pointer_button_down_on() && !self.temp_name.is_empty() && !self.temp_size.is_empty() && self.temp_size.parse::<f32>().is_ok()  {
                    self.spacers.push(Spacer { name: self.temp_name.to_string(), size: self.temp_size.parse().unwrap(), used: true });
                    self.temp_name = Default::default();
                    self.temp_size = Default::default();
                }
            });
        });
    }
}

#[derive(Clone)]
struct Spacer {
    size: f32,
    used: bool,
    name: String,
}