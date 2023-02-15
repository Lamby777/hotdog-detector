// typedefs, structs, impls, etc.
// stuff that would take up too much space in main

use std::{fs, path::{PathBuf, Path}};
use serde::{Serialize, Deserialize};
use crate::{INTERMEDIATE_LAYERS, INPUT_NODES, ILAYER_NODES};

pub type IDFC<T> = Result<T, Box<dyn std::error::Error>>;
pub type WeightPair = (f64, f64); // (m, b) where y = mx + b
pub type NodeBoxed = u16; // Value that neurons keep track of

#[derive(Serialize, Deserialize)]
pub struct NeuralNet {
	input_layer:	NeuralNetLayer,
	hidden_layers:	Vec<NeuralNetLayer>,
	generation:		u64,
}

impl NeuralNet {
	pub fn new(hidden_layers: usize) -> Self {
		Self {
			input_layer:	NeuralNetLayer::new(INPUT_NODES),
			hidden_layers:	vec![NeuralNetLayer::new(ILAYER_NODES); INTERMEDIATE_LAYERS],
			generation:		0,
		}
	}

	/*
	* Take data for the input layer, and run it through
	* the network to get the resulting output layer data
	*
	* evaluate(Input Data) -> Output Data
	*/
	pub fn evaluate(input: Vec<f64>) -> NeuralNetLayer {
		todo!()
	}

	pub fn load_path<P: AsRef<Path>>(path: P) -> Option<Self> {
		let data = fs::read_to_string(path).ok()?;
		serde_json::from_str(&data).ok()?
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NeuralNetLayer {
	neurons:	Vec<NeuralNetNode>
}

impl NeuralNetLayer {
	pub fn new(node_ct: usize) -> Self {
		Self {
			neurons: vec![NeuralNetNode::new(); node_ct],
		}
	}
}

// the "neurons" of the brain
#[derive(Serialize, Deserialize, Clone)]
pub struct NeuralNetNode {
	value:	NodeBoxed,
	weight:	WeightPair,
}

impl NeuralNetNode {
	pub fn new() -> Self {
		Self {
			value:	0,
			weight:	(1.0, 1.0),
		}
	}

	pub fn fire(&self) {
		// calculate signals to send forward
	}

	pub fn apply_offset(&mut self, offset: WeightPair) {
		self.weight = (
			self.weight.0 + offset.0,
			self.weight.1 + offset.1,
		)
	}
}

#[derive(Serialize, Deserialize)]
pub struct DatasetRow {
	img_path:	PathBuf,
	is_hotdog:	bool
}
