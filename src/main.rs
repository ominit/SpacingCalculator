#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::{self, TextStyle, FontFamily::Proportional}, epaint::FontId, IconData};
use serde::{Deserialize, Serialize};
use self_update::cargo_crate_version;

fn update_app() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("ominit")
        .repo_name("SpacingCalculator")
        .bin_name(if cfg!(windows) {"SpacingCalculator.exe"} else {"SpacingCalculator"} )
        .no_confirm(true)
        .show_output(false)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

fn main() {
    if let Err(e) = update_app() {
        println!("[ERROR] {}", e);
        ::std::process::exit(1);
    }

    let mut options = eframe::NativeOptions::default();

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/spacer.png");
    let rgb: Vec<u8> = image::open(path).unwrap_or_default().into_bytes();
    options.icon_data = Option::<IconData>::Some(IconData { rgba: rgb, width: 1885, height: 1885 });

    if let Err(e) = eframe::run_native(
        "Spacing Calculator",
        options,
        Box::new(|_cc| Box::new(SpacingCalculatorApp::new(_cc))),
    ) {
        println!("[ERROR] {}", e);
        ::std::process::exit(1);
    }
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct SpacingCalculatorApp {
    input: String,
    temp_name: String,
    temp_size: String,
    spacers: Vec<Spacer>,
    prev_outputs: Vec<String>,
    current_page: Page,
}

impl Default for SpacingCalculatorApp {
    fn default() -> Self {
        let mut spacer_list: Vec<Spacer> = Vec::new();
        spacer_list.push(Spacer { name: "1/2 Nylon".to_string(), size: 0.5, used: true });
        spacer_list.push(Spacer { name: "3/8 Nylon".to_string(), size: 0.375, used: true });
        spacer_list.push(Spacer { name: "1/4 Nylon".to_string(), size: 0.25, used: true });
        spacer_list.push(Spacer { name: "1/8 Nylon".to_string(), size: 0.125, used: true });
        spacer_list.push(Spacer { name: "Thick Teflon".to_string(), size: 0.0625, used: true });
        spacer_list.push(Spacer { name: "Black Steel".to_string(), size: 0.03125, used: true });
        spacer_list.push(Spacer { name: "Thin Teflon".to_string(), size: 0.03125, used: true });
        spacer_list.push(Spacer { name: "Small Black".to_string(), size: 0.181, used: true });
        spacer_list.push(Spacer { name: "Large Black".to_string(), size: 0.315, used: true });
        spacer_list.push(Spacer { name: "Thick Steel".to_string(), size: 0.0392, used: true });
        spacer_list.push(Spacer { name: "Thin Steel".to_string(), size: 0.0215, used: true });
        spacer_list.sort_by(|a, b| b.size.total_cmp(&a.size));

        SpacingCalculatorApp {
            input: String::default(),
            temp_name: String::default(),
            temp_size: String::default(),
            spacers: spacer_list,
            prev_outputs: Vec::new(),
            current_page: Page::MAIN,
        }
    }
}

impl SpacingCalculatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Body, FontId::new(20.0, Proportional)),
            (TextStyle::Button, FontId::new(20.0, Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for SpacingCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        central_panel(self, ctx, frame);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_egui_memory(&self) -> bool { true }
    fn persist_native_window(&self) -> bool { true }
}

fn central_panel(app: &mut SpacingCalculatorApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    match app.current_page {
        Page::MAIN => main_page(app, ctx),
        Page::SETTINGS => settings_page(app, ctx, frame),
    }
}

#[derive(Deserialize, Serialize)]
enum Page {
    MAIN,
    SETTINGS,
}

fn main_page(app: &mut SpacingCalculatorApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.add(egui::TextEdit::singleline(&mut app.input).hint_text("Spacing"));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| if ui.button("\u{2699}").clicked() {
                app.current_page = Page::SETTINGS;
            });
        });
        if !app.input.is_empty() && app.input.parse::<f64>().is_ok() {
            let mut used_input: f64 = 0.0;
            let input_num = app.input.parse::<f64>().unwrap();
            let mut sinputs: Vec<f64> = vec![];
            for _i in &mut app.spacers {
                sinputs.push(0.0);
            }
            for i in 0..app.spacers.len() {
                while app.spacers[i].size <= input_num - used_input && app.spacers[i].used {
                    sinputs[i] += 1.0;
                    used_input += app.spacers[i].size;
                }
            }
            ui.label("Off by ".to_string() + &(input_num - used_input).to_string());
            for i in 0..app.spacers.len() {
                if sinputs[i] > 0.0 {
                    ui.label(sinputs[i].to_string() + " " + &app.spacers[i].name.to_string() + " " + &app.spacers[i].size.to_string());
                }
            }
            if ui.button("Save Output").is_pointer_button_down_on() {
                let mut spacing_output: String = app.input.to_string() + " in";
                spacing_output += &("\n\tOff by ".to_string() + &(input_num - used_input).to_string());
                for i in 0..app.spacers.len() {
                    if sinputs[i] > 0.0 {
                        spacing_output += &("\n\t".to_string() + &sinputs[i].to_string() + " " + &app.spacers[i].name.to_string() + " " + &app.spacers[i].size.to_string());
                    }
                }
                app.prev_outputs.push(spacing_output);
                app.input = Default::default();
            }
        }
        if !app.prev_outputs.is_empty() {
            ui.collapsing("Previous Outputs", |ui| {
                let mut prev_output_str: String = Default::default();
                for i in 0..app.prev_outputs.len() - 1 {
                    prev_output_str += &(app.prev_outputs[i].to_string() + &"\n".to_string());
                }
                prev_output_str += &app.prev_outputs[app.prev_outputs.len() - 1].to_string();
                ui.label(prev_output_str);
            });
        }
        ui.separator();
        ui.collapsing("Spacers", |ui| {
            let mut j: usize = 0;
            let mut checkboxes = vec![];
            for i in &mut app.spacers {
                checkboxes.push(ui.checkbox(&mut i.used, i.size.to_string() + " " + &i.name.to_string()));
            }
            for i in checkboxes {
                if i.context_menu(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut app.spacers.get_mut(j).unwrap().name).hint_text("Name"));
                    let mut size_str = app.spacers.get(j).unwrap().size.to_string();
                    let size_edit = ui.add(egui::TextEdit::singleline(&mut size_str).hint_text("Size"));
                    if size_edit.lost_focus() {
                        ui.close_menu();
                        app.spacers.sort_by(|a, b| b.size.total_cmp(&a.size));
                    } else if size_edit.changed() && size_str.parse::<f64>().is_ok(){
                        app.spacers.get_mut(j).unwrap().size = size_str.parse::<f64>().unwrap();
                    }
                    if ui.button("Delete").clicked() {
                        ui.close_menu();
                        app.spacers.swap_remove(j);
                        app.spacers.sort_by(|a, b| b.size.total_cmp(&a.size));
                    }
                }).clicked_elsewhere() {
                    app.spacers.sort_by(|a, b| b.size.total_cmp(&a.size));
                }
                j+=1;
            }
        });
        ui.collapsing("Add Spacer", |ui| {
            ui.add(egui::TextEdit::singleline(&mut app.temp_name).hint_text("Name"));
            ui.add(egui::TextEdit::singleline(&mut app.temp_size).hint_text("Size"));
            if ui.button("Add Spacer").is_pointer_button_down_on() && !app.temp_name.is_empty() && !app.temp_size.is_empty() && app.temp_size.parse::<f64>().is_ok()  {
                app.spacers.push(Spacer { name: app.temp_name.to_string(), size: app.temp_size.parse().unwrap(), used: true });
                app.spacers.sort_by(|a, b| b.size.total_cmp(&a.size));
                app.temp_name = Default::default();
                app.temp_size = Default::default();
            }
        });
    });
}

fn settings_page(app: &mut SpacingCalculatorApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Settings");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| if ui.button("\u{2699}").clicked() {
                app.current_page = Page::MAIN;
            });
        });
        ui.separator();
        
        if ui.button("Reset").clicked() {reset(app);}
    });
}

fn reset(app: &mut SpacingCalculatorApp) {
    *app = SpacingCalculatorApp::default();
}

#[derive(Clone, Deserialize, Serialize)]
struct Spacer {
    size: f64,
    used: bool,
    name: String,
}
