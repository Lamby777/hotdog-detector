/*
* Subcommands
*/

use std::fs;

use crate::{IDFC, classes::DatasetRow};

// train command code
pub fn train(m_per_gen: u32) -> IDFC<()> {
	todo!()
}

// test a single image
pub fn feed_img() {
	//
}

fn read_data_csv() -> IDFC<DatasetRow> {
	let mut csv_reader = csv::Reader::from_path("")?;

	let dataset = fs::read_dir("./dataset").expect("Folder `dataset` required for training!");

	for record in csv_reader.deserialize() {
		let record: DatasetRow = record?;
	}
	
	todo!()
}
