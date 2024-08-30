use eframe::egui;

#[derive(Default)]
pub struct TemplateApp {}

impl TemplateApp {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for TemplateApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Hello, World!");
		});
	}
}
