use super::*;
use crate::{OptionDetail, Options};
use indexmap::IndexMap;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver};
use std::thread::spawn;

pub type Changes = IndexMap<String, String>;

#[derive(Debug, Default)]
pub enum SetupScreen {
	#[default] Start,
	Command(Receiver<String>),
	Parse(String),
	Selection(Options),
}

#[derive(Debug, Default)]
pub struct SetupState {
	pub path: String,
	pub changes: Changes,
	pub cmd: String,
	pub screen: SetupScreen,
}

impl SetupState {
	fn compute_command(&mut self) {
		let mut cmd = String::from("scons");
		for (k, v) in &self.changes {
			cmd += &format!(" {k}={}", v.replace(' ', "_"));
		}
		self.cmd = cmd;
	}
}

pub fn show(state: &mut SetupState, ctx: &Context) -> Option<AppState> {
	TopBottomPanel::bottom("cmd")
		.max_height(150.)
		.resizable(true)
		.show(ctx, |ui| {
			ScrollArea::vertical().show(ui, |ui| {
				ui.add_space(6.);

				ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
					if ui.button("📋 Copy").clicked() {
						ui.ctx().copy_text(state.cmd.clone());
					}
					Frame::default()
						.inner_margin(6.)
						.rounding(6.)
						.fill(Color32::from_rgb(22, 22, 22))
						.stroke(Stroke::new(2., Color32::from_rgb(15, 15, 15)))
						.show(ui, |ui| {
							let text = egui::RichText::new(&state.cmd).monospace();
							ui.add_sized(ui.available_size(), Label::new(text));
						});
				});

			});
		});
	CentralPanel::default().show(ctx, |ui| {
		ScrollArea::vertical()
			.auto_shrink(false)
			.show(ui, |ui| {
				ui.label(&state.path);
				match &state.screen {
					SetupScreen::Start => {
						let btn = Button::new("Load compilation options");
						if ui.add_sized(Vec2::new(200., 30.), btn).clicked() {
							let (tx, rx) = channel();

							let path = state.path.clone() + "/godot";
							spawn(move || {
								let output = Command::new("scons")
									.arg("--help")
									.current_dir(path)
									.output()
									.unwrap();

								tx.send(String::from_utf8(output.stdout).unwrap()).unwrap();
							});

							state.screen = SetupScreen::Command(rx);
						}
					}
					SetupScreen::Command(rx) => {
						ui.horizontal(|ui| {
							ui.spinner();
							ui.label(egui::RichText::new("scons --help").monospace());
						});
						if let Ok(output) = rx.try_recv() {
							state.screen = SetupScreen::Parse(output);
						}
					}
					SetupScreen::Parse(string) => {
						ui.label("Parsing...");

						let mut options = Options::default();
						let mut chunks = string.split("\r\n\r\n")
							.skip(1)
							.map(|s| s.split('\n').map(|x| x.trim_ascii_end()).collect::<Vec<_>>())
							.collect::<Vec<_>>();
						chunks.pop();

						ui.label(format!("{chunks:#?}"));

						for chunk in chunks {
							let mut option = OptionDetail::default();
							let mut chunk = chunk.into_iter().map(|x| x.trim_ascii());

							let header = chunk.next().unwrap();
							let (name, right) = header.split_once(": ").unwrap();
							if let Some((desc, values)) = right.rsplit_once(" (") {
								option.description = desc.to_owned();

								option.values = values.split_once(')').unwrap().0
									.split('|')
									.map(|x| x.to_owned())
									.collect::<Vec<String>>();

								for line in chunk {
									let (k, v) = line.split_once(':').unwrap();
									let v = match v.trim_ascii_start() {
										"True" => "yes",
										"False" => "no",
										_ => v,
									};
									if v.is_empty() { continue; }

									match k {
										"default" => option.default = Some(v.to_owned()),
										"actual" => option.actual = Some(v.to_owned()),
										"aliases" => option.aliases.push(v.to_owned()),
										_ => println!("{k:?}"),
									}
								}
							} else {
								option.description = right.to_owned();
							}

							options.options.insert(name.to_owned(), option);
						}

						state.screen = SetupScreen::Selection(options);
					}
					SetupScreen::Selection(options) => {
						let mut recompute_cmd = false;
						
						for (k, v) in &options.options {
							ui.group(|ui| {
								ui.allocate_space(Vec2::new(ui.available_width(), 0.));
								ui.heading(k);
								ui.label(&v.description);
								ui.small(if v.values.is_empty() { "🚫".into() } else { v.values.join(", ") });
							});

							if let Some(value) = state.changes.get_mut(k) {
								if ui.add(TextEdit::singleline(value).desired_width(f32::INFINITY)).changed() {
									recompute_cmd = true;
									if value.trim_ascii().is_empty() {
										state.changes.shift_remove(k);
									}
								}
							} else {
								let mut value = String::new();
								if ui.add(TextEdit::singleline(&mut value).hint_text(v.default.as_ref().unwrap_or(&"".into())).desired_width(f32::INFINITY)).changed() {
									recompute_cmd = true;
									if !value.trim_ascii().is_empty() {
										state.changes.insert(k.to_owned(), value);
									}
								}

							}
						}
						
						if recompute_cmd {
							state.compute_command();
						}
					}
				}
			});
	});

	None
}
