mod env_var;
mod functions;
mod impostazioni;
use eframe::*;
use eframe::emath::Vec2;
mod home;
use home::*;

// il problema è che collapsing header è sempre in alto a sinistra, non si sposta in centro! Dovrei trovare un modo con window o con qualche altro ui generator

fn main() {
    let home = Home::new();
	let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(200.* home.size, 200.* home.size));
    native_options.resizable = false;
    run_native("Magic", native_options, Box::new(|_cc: &CreationContext| Box::new(home)));
}
