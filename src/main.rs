#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod options;
mod views;

use eframe::egui;
use indexmap::IndexMap;


fn main() -> eframe::Result {
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([400.0, 300.0])
			.with_min_inner_size([300.0, 220.0]),
		..Default::default()
	};
	eframe::run_native(
		"godot-scons-gui",
		native_options,
		Box::new(|cc| Ok(Box::new(app::GodotSconsGUI::new(cc)))),
	)
}

#[derive(Debug, Default)]
struct Options {
	pub options: IndexMap<String, OptionDetail>,
	// todo: categories
}

#[derive(Debug, Default)]
struct OptionDetail {
	pub description: String,
	pub values: Vec<String>,
	pub default: Option<String>,
	pub actual: Option<String>,
	pub aliases: Vec<String>,
}
