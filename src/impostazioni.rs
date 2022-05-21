use eframe::*;
use eframe::epaint::Color32;
use std::fs::File;
use std::io::Write;
use egui::FontDefinitions;
use std::io::Read;
use eframe::epaint::FontFamily;



pub fn impostazioni(ui: &mut egui::Ui, option: &mut u8, colore: &mut Color32, size: &mut f32, window_size: &mut Option<egui::Vec2>) {
    *window_size = Some(egui::vec2(400. * *size, 400. * *size)); // nelle impostazioni la finestra è più grande
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(12. * *size);


//    println!("{:?}", ctx.used_size());
/*         match window_size {     // it's never defined
            Some(value) => print!("{}, {}\n", value.x, value.y),
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
/*         ui.columns(5, |ui| {
            if ui[0].small_button("Indietro").clicked() {
                let info = Info::new();
                *size = info.size;
                *colore = info.color();
                *option = 0;
            }
            if ui[2].button("Salva ed esci").clicked() {
                salva_modifiche(&size, &colore);
                *option = 0;
            }
        }); */
        if ui.button("Salva ed esci").clicked() {
            salva_modifiche(&size, &colore);
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

pub fn salva_modifiche(size: &f32, colore: &Color32) { // color_edit_button_rgb() non modifica l'array che gli viene date
    let mut file = File::create("./.info.stg").unwrap();
    file.write_all(format!("{}\n{}\n{}\n{}\n{}", 
        *size, colore.r(), colore.g(), colore.b(), colore.a()).as_bytes()).unwrap();    // si poteva anche usafe Colors32::to_array()
}

// installa il font che mi piace
pub fn font(size: &f32) -> eframe::egui::FontDefinitions {
    let mut fonts = FontDefinitions::default();
    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
    eframe::egui::FontData::from_static(include_bytes!("./../cmunbx.ttf"))); // .ttf and .otf supported

    let mut font= fonts.font_data.get_mut("my_font").unwrap();
    font.tweak.scale = 1.7 * size;
    
    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("my_font".to_owned());
    fonts
}

// fetch delle dimensioni e del colore
pub struct Info {
    pub size: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub font: egui::FontDefinitions
}

impl Info {
    pub fn new() -> Info {
        let mut file = std::fs::OpenOptions::new().read(true).open("./.info.stg").expect("settings not found");
        let mut lettura = format!("");
        let _x = file.read_to_string(&mut lettura);
        let vett = 
        lettura.split('\n').collect::<Vec<&str>>();
        let dim = vett[0].parse::<f32>().unwrap();

        Info { 
            size: dim, 
            r: vett[1].parse::<u8>().unwrap(), 
            g: vett[2].parse::<u8>().unwrap(), 
            b: vett[3].parse::<u8>().unwrap(),
            a: vett[4].parse::<u8>().unwrap(),
            font: font(&dim)
        }
    }

    pub fn color(&self) -> Color32 {
        Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a)
    }
}