use std::io::Read;
use eframe::epaint::FontFamily;
use eframe::egui::FontDefinitions;
use eframe::epaint::Color32;

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
    pub a: u8
}

impl Info {
    pub fn new() -> Info {
        let mut file = std::fs::File::open("./.info.stg").expect("settings not found");
        let mut lettura = format!("");
        let _x = file.read_to_string(&mut lettura);
        let vett = 
        lettura.split('\n').collect::<Vec<&str>>();

        Info { 
            size: vett[0].parse::<f32>().unwrap(), 
            r: vett[1].parse::<u8>().unwrap(), 
            g: vett[2].parse::<u8>().unwrap(), 
            b: vett[3].parse::<u8>().unwrap(),
            a: vett[4].parse::<u8>().unwrap()
        }
    }

    pub fn color(&self) -> Color32 {
        Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a)
    }
}