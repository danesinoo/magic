use eframe::{*, Frame};
use eframe::egui::{Frame as Sfondo};
use eframe::epaint::Color32;
use crate::{functions::*};

pub struct Home {
    pub size: f32,
    pub fill: Color32,
    pub option: u8
}

impl Home {
    pub fn new() -> Self {
        let info = Info::new();
        Home {
            size: info.size,
            fill: info.color(),
            option: 0
        }
    }
}

impl eframe::App for Home {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {

        ctx.set_fonts(font(&self.size));  // imposto il font

        frame.set_window_size(egui::vec2(200. * self.size, 200. * self.size));

        let mut sfondo: Sfondo = Sfondo::default();
        sfondo = sfondo.fill(self.fill); // imposto il colore
            
        egui::CentralPanel::default().frame(sfondo).show(ctx, |ui| {
            // allineo tutto al centro
//            ui.horizontal_centered(|ui| {
                // impilo i bottoni uno sotto l'altro
                ui.vertical_centered(|ui| {
                    // spazio
                    if self.option == 0 {
                        home(ui, &mut self.option, &self.size);
                    }

                    else if self.option == 1 {
                        registra(ui, &mut self.option, &self.size);
                    }

                    else if self.option == 2{
                        analisi(ui, &mut self.option, &self.size);
                    }

                    else if self.option == 3 {
                        crate::impostazioni::impostazioni(ui, &mut self.option, &mut self.fill, &mut self.size, &mut frame.output.window_size);
                    }
                });
            });
//        });
    }
}

fn home(ui: &mut egui::Ui, option: &mut u8, size: &f32) {
    ui.add_space(12. * size);
    if ui.button("Registra dati").clicked() {
        *option = 1;
    }
    ui.add_space(10. * size);
    if ui.button("Analisi dei Dati").clicked() {
        *option = 2;
    }
    ui.add_space(10. * size);
    if ui.button("Impostazioni").clicked() {
        *option = 3;
    }
    ui.add_space(10. * size);
    if ui.button("Esci").clicked() {
        std::process::exit(0);
    }
}

fn registra(ui: &mut egui::Ui, option: &mut u8, size: &f32) {
    ui.add_space(12. * size);
    if ui.button("Nuova Cartella").clicked() {
    }
    ui.add_space(10. * size);
    if ui.button("Copia Partite").clicked() {
    }
    ui.add_space(10. * size);
    if ui.button("Apri Excell Partite").clicked() {
    }
    ui.add_space(10. * size);
    if ui.button("Indietro").clicked() {
        *option = 0;
    }
}

fn analisi(ui: &mut egui::Ui, option: &mut u8, size: &f32) {
    ui.add_space(10. * size);
    if ui.button("Analisi dei Dati").clicked() {
    }
    ui.add_space(10. * size);
    if ui.button("Mostra Dati").clicked() {
    }
    ui.add_space(10. * size);
    if ui.button("Indietro").clicked() {
        *option = 0;
    }
}