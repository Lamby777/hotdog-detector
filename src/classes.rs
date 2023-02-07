// typedefs, structs, impls, etc.
// stuff that would take up too much space in main

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub type IDFC<T> = Result<T, Box<dyn std::error::Error>>;

pub struct NeuralNet {
	hidden_layers:	Vec<NeuralNetLayer>
}

pub struct NeuralNetLayer {
	neurons:	Vec<NeuralNetNode>
}

// the "neurons" of the brain
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
