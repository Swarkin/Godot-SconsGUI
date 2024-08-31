#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod states;
mod options;

use eframe::egui;

fn main() -> eframe::Result {
	let options = options::load().unwrap();
	
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([400.0, 300.0])
			.with_min_inner_size([300.0, 220.0]),
		..Default::default()
	};
	eframe::run_native(
		"godot-scons-gui",
		native_options,
		Box::new(|cc| Ok(Box::new(app::GodotSconsGUI::new(cc, options)))),
	)
}
