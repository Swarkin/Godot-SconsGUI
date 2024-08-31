use crate::options::Options;
use crate::states::*;
use eframe::egui;
use egui::{Button, Color32, Vec2};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub struct GodotSconsGUI {
	options: Options,
	state: AppState,
}

impl GodotSconsGUI {
	pub fn new(_cc: &eframe::CreationContext<'_>, options: Options) -> Self {
		Self { options, state: AppState::Start }
	}
}

impl eframe::App for GodotSconsGUI {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		#[cfg(debug_assertions)] {
			egui::TopBottomPanel::bottom("debug").show(ctx, |ui| {
				ui.label(format!("{:?}", &self.state));

				ui.horizontal(|ui| {
					if let Some(s) = {
						let mut s = None;
						if ui.button("Start").clicked() { s = Some(AppState::Start); }
						if ui.button("Clone").clicked() { s = Some(AppState::CLONE); }
						if ui.button("Setup").clicked() { s = Some(AppState::SETUP); }
						if ui.button("Compile").clicked() { s = Some(AppState::COMPILE); }
						s
					} { self.state = s; }
				});
			});
		}

		match &mut self.state {
			AppState::Start => {
				egui::CentralPanel::default().show(ctx, |ui| {
					ui.vertical_centered(|ui| {
						let btn = Button::new("➕ Create");
						if ui.add_sized(Vec2::new(180., 80.), btn).clicked() {
							self.state = AppState::CLONE;
						}
					});
				});
			}
			AppState::Clone(state) => {
				// todo: add selectable github tag for clone

				egui::CentralPanel::default().show(ctx, |ui| {
					let btn = Button::new("📋 Clone godotengine/godot");
					ui.add_enabled_ui(state.rx.is_none(), |ui| {
						if ui.add_sized(Vec2::new(220., 60.), btn).clicked() {
							state.status = None;

							let (tx, rx) = channel::<i32>();
							state.rx = Some(rx);

							spawn(move || {
								let status = Command::new("git")
									.args(["clone",
										"--depth", "1",
										"--branch", "4.3-stable",
										"https://github.com/godotengine/godot"])
									.current_dir("C:/Users/Me/Desktop/test")
									.status().unwrap()
									.code().unwrap();

								tx.send(status).unwrap();
								println!("exit");
							});
						}
					});

					if let Some(rx) = &state.rx {
						if let Ok(status) = rx.try_recv() {
							state.status = Some(status);
							state.rx = None;
						}

						ui.horizontal(|ui| {
							ui.spinner();
							ui.label("Cloning...");
						});
					}

					if let Some(status) = state.status {
						if status == 0 {
							ui.add_space(10.);
							ui.colored_label(Color32::LIGHT_GREEN, "✅ Cloned successfully.");

							let btn = Button::new("Continue ➡");
							if ui.add_sized(Vec2::new(100., 40.), btn).clicked() {
								self.state = AppState::SETUP;
							}
						} else {
							ui.add_space(10.);
							ui.colored_label(Color32::LIGHT_RED, format!("❎ Clone failed with status {status}."));
						}
					}
				});
			}
			AppState::Setup(_state) => {
				egui::CentralPanel::default().show(ctx, |ui| {
					egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
						for (k, v) in &self.options.options {
							ui.group(|ui| {
								ui.allocate_space(Vec2::new(ui.available_width(), 0.));
								ui.heading(k);
								ui.label(&v.description);
							});
							ui.add_space(6.);
						}
					});
				});
			}
			AppState::Compile(_state) => {
				egui::CentralPanel::default().show(ctx, |ui| {
					ui.label("Compiling...");
				});
			}
		}
	}
}
