// typedefs, structs, impls, etc.
// stuff that would take up too much space in main

use std::path::{PathBuf, Path};
use serde::{Serialize, Deserialize};

pub type IDFC<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
pub struct NeuralNet {
	hidden_layers:	Vec<NeuralNetLayer>,
	generation:		u64,
}

impl NeuralNet {
	pub fn new() -> Self {
		Self {
			hidden_layers:	todo!(),
			generation:		todo!(),
		}
	}

	pub fn load_path<P: AsRef<Path>>(path: P) -> Option<Self> {
		todo!()
	}
}

#[derive(Serialize, Deserialize)]
pub struct NeuralNetLayer {
	neurons:	Vec<NeuralNetNode>
}

// the "neurons" of the brain
#[derive(Serialize, Deserialize)]
pub struct NeuralNetNode {
	// (m, b)
	weight:	(f64, f64),
	offset: (f64, f64),
}

impl NeuralNetNode {
	pub fn result(&self) {
		// calculate signal to send forward
	}

	pub fn apply_offset(&mut self) {
		self.weight = (
			self.weight.0 + self.offset.0,
			self.weight.1 + self.offset.1,
		);

		self.offset = (
			0.0, 0.0
		);
	}
}

#[derive(Serialize, Deserialize)]
pub struct DatasetRow {
	img_path:	PathBuf,
	is_hotdog:	bool
}
