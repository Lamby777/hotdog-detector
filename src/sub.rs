/*
* Subcommands
*/

use std::fs;

use crate::{IDFC, classes::{DatasetRow, NeuralNet}, INTERMEDIATE_LAYERS};

// train command code
pub fn train(m_per_gen: u32) -> IDFC<()> {
	let training_data	= read_data_csv();
	let model			= load_model();

	Ok(())
}

fn read_data_csv() -> IDFC<Vec<DatasetRow>> {
	let mut csv_reader = csv::Reader::from_path("./training_data.csv")?;

	let dataset = fs::read_dir("./dataset").expect("Folder `dataset` required for training!");

	// maybe turn this part into an iter method chain and a .collect() later
	let mut res = vec![];

	for record in csv_reader.deserialize() {
		let record: DatasetRow = record?;
		res.push(record);
	}
	
	Ok(res)
}

fn load_model() -> NeuralNet {
	NeuralNet::load_path("./model.json")
		.unwrap_or(NeuralNet::new(INTERMEDIATE_LAYERS))
}
