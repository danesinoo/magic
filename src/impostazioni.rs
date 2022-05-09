use eframe::*;
use eframe::epaint::Color32;
use std::fs::File;
use std::io::Write;


pub fn impostazioni(ui: &mut egui::Ui, option: &mut u8, colore: &mut Color32, size: &mut f32, window_size: &mut Option<egui::Vec2>) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(12. * *size);

        *window_size = Some(egui::vec2(400. * *size, 400. * *size));
       
    /*  match window_size {     // it's never defined
            Some(_value) => print!("{}, {}\n", window_size.unwrap().x, window_size.unwrap().y),
            _ => {}
        }
    */
        

        egui::CollapsingHeader::new("Leggibilità").show(ui, |ui| {
            ui.add_space(10. * *size);
            ui.columns(8, |ui| {
                if ui[3].small_button("-").clicked() {
                        *size -= 0.1;
                };
                if ui[4].small_button("+").clicked() {
                        *size += 0.1;
                }
            });
            ui.add_space(3. * *size);
            ui.small("Le dimensioni della finestra non cambiano automaticamente fino al ravvio");
        });
       
        ui.add_space(12. * *size);
        egui::CollapsingHeader::new("Sfondo").show(ui, |ui| {
        cambia_colore(ui, colore, size);
        });
        ui.add_space(10. * *size);
        if ui.button("Indietro").clicked() {
            modifica_impostazioni(&size, &colore);
            *option = 0;
        }
        ui.label(" ");

    });
}   
    

fn cambia_colore(ui: &mut egui::Ui, colore: &mut Color32, size: &f32) {
    ui.add_space(12. * size);
    ui.color_edit_button_srgba(colore);
    ui.add_space(10. * size);
}

pub fn modifica_impostazioni(size: &f32, colore: &Color32) { // color_edit_button_rgb() non modifica l'array che gli viene date
    let mut file = File::create("./.info.stg").unwrap();
    file.write_all(format!("{}\n{}\n{}\n{}\n{}", 
        *size, colore.r(), colore.g(), colore.b(), colore.a()).as_bytes()).unwrap();    // si poteva anche usafe Colors32::to_array()
}