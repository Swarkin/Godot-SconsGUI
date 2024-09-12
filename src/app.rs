use crate::views::*;
use eframe::egui;
use egui::Color32;

#[derive(Default)]
pub struct GodotSconsGUI {
	state: AppState,
}

#[derive(Debug)]
pub enum AppState {
	Start(StartState),
	Clone(CloneState),
	Setup(SetupState),
	Build(BuildState),
}


impl AppState {
	pub fn start() -> Self {
		AppState::Start(Default::default())
	}
	pub fn clone() -> Self {
		AppState::Clone(Default::default())
	}
	pub fn setup() -> Self {
		AppState::Setup(Default::default())
	}
	pub fn build() -> Self {
		AppState::Build(Default::default())
	}
}

impl Default for AppState {
	fn default() -> Self {
		Self::Start(Default::default())
	}
}


impl GodotSconsGUI {
	pub fn new(cc: &eframe::CreationContext) -> Self {
		cc.egui_ctx.style_mut(|style| {
			style.visuals.override_text_color = Some(Color32::from_rgb(170, 170, 170));
		});
		Default::default()
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
						if ui.button("Start").clicked() { s = Some(AppState::start()); }
						if ui.button("Clone").clicked() { s = Some(AppState::clone()); }
						if ui.button("Setup").clicked() { s = Some(AppState::setup()); }
						if ui.button("Build").clicked() { s = Some(AppState::build()); }
						s
					} { self.state = s; }
				});
			});
		}

		let new_state = match &mut self.state {
			AppState::Start(state) => start::show(state, ctx),
			AppState::Clone(state) => clone::show(state, ctx),
			AppState::Setup(state) => setup::show(state, ctx),
			AppState::Build(state) => build::show(state, ctx),
		};

		if let Some(s) = new_state {
			#[cfg(debug_assertions)] println!("Transitioning to {s:?}");
			self.state = s;
		}
	}
}
