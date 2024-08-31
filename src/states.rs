use std::sync::mpsc::Receiver;

#[derive(Debug, Default)]
pub enum AppState {
	#[default] Start,
	Clone(CloneState),
	Setup(SetupState),
	Compile(CompileState),
}

impl AppState {
	//pub const START: Self = Self::Start;
	pub const CLONE: Self = Self::Clone(CloneState {rx: None, status: None});
	pub const SETUP: Self = Self::Setup(SetupState {});
	pub const COMPILE: Self = Self::Compile(CompileState {});
}


#[derive(Debug, Default)]
pub struct CloneState {
	pub rx: Option<Receiver<i32>>,
	pub status: Option<i32>,
}

#[derive(Debug, Default)]
pub struct SetupState {

}

#[derive(Debug, Default)]
pub struct CompileState {

}
