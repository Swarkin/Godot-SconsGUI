pub mod start;
pub mod clone;
pub mod setup;
pub mod build;

pub use build::BuildState;
pub use clone::CloneState;
pub use setup::SetupState;

use crate::app::AppState;
use eframe::egui;
use egui::{Button, Color32, Context, Frame, Stroke, TextEdit, Vec2};
use egui::{CentralPanel, ScrollArea, TopBottomPanel};