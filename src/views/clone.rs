use super::*;
use std::process::Command;
use std::sync::mpsc::Receiver;

#[derive(Debug, Default)]
pub struct CloneState {
	pub rx: Option<Receiver<i32>>,
	pub status: Option<i32>,
}

pub fn show(state: &mut CloneState, ctx: &egui::Context) -> Option<AppState> {
	// todo: add selectable github tag for clone

	CentralPanel::default()
		.show(ctx, |ui| {
			let btn = Button::new("📋 Clone godotengine/godot");
			ui.add_enabled_ui(state.rx.is_none(), |ui| {
				if ui.add_sized(Vec2::new(220., 60.), btn).clicked() {
					state.status = None;

					let (tx, rx) = std::sync::mpsc::channel::<i32>();
					state.rx = Some(rx);

					std::thread::spawn(move || {
						let status = Command::new("git")
							.args([
								"clone",
								"--depth",
								"1",
								"--branch",
								"4.3-stable",
								"https://github.com/godotengine/godot",
							])
							.current_dir("D:/Godot/test")
							.status()
							.unwrap()
							.code()
							.unwrap();

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
						return Some(AppState::setup());
					}
				} else {
					ui.add_space(10.);
					ui.colored_label(
						Color32::LIGHT_RED,
						format!("❎ Clone failed with status {status}."),
					);
				}
			}
			None
		})
		.inner
}
