use super::*;

#[derive(Debug, Default)]
pub struct BuildState {
	pub path: String,
}

pub fn show(state: &mut BuildState, ctx: &Context) -> Option<AppState> {
	CentralPanel::default().show(ctx, |_ui| {});
	None
}
