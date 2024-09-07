use super::*;

pub fn show(ctx: &Context) -> Option<AppState> {
	CentralPanel::default()
		.show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				let btn = Button::new("➕ Create");
				if ui.add_sized(Vec2::new(180., 80.), btn).clicked() {
					Some(AppState::clone())
				} else {
					None
				}
			})
		}).inner.inner
}
