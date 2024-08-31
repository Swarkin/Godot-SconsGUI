#![allow(unused)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Options {
	pub options: HashMap<String, OptionDetail>,
}

#[derive(Debug, Deserialize)]
pub struct OptionDetail {
	pub description: String,
	pub values: Vec<String>,
	pub default: String,
}

pub fn load() -> std::io::Result<Options> {
	let string = std::fs::read_to_string("./src/options.toml")?;
	Ok(toml::from_str::<Options>(&string).unwrap())
}
