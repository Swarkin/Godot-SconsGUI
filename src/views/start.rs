use super::*;

#[derive(Debug, Default)]
pub struct StartState {
	pub path: Option<String>,
}

pub fn show(state: &mut StartState, ctx: &Context) -> Option<AppState> {
	CentralPanel::default()
		.show(ctx, |ui| {
			let btn_size = Vec2::new(180., 80.);
			ui.vertical_centered(|ui| {
				if ui.add_sized(btn_size, Button::new("📁 Choose Directory")).clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_folder() {
						state.path = Some(path.display().to_string());
					}
				}
				
				ui.add_enabled_ui(state.path.is_some(), |ui| {
					ui.add_space(8.);
					
					return if ui.add_sized(btn_size, Button::new("➕ Create")).clicked() {
						let path = state.path.as_ref().unwrap().to_owned();
						Some(AppState::Clone(CloneState { path, ..Default::default() }))
					} else { None }
				}).inner
			})
		}).inner.inner
}
