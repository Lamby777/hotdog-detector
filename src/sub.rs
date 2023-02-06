/*
* Subcommands
*/

use std::fs;

// train command code
pub fn train(m_per_gen: u32) {
	let csv_data = None;
	let dataset = fs::read_dir("./dataset").expect("Folder `dataset` required for training!");
}

// test a single image
pub fn feed_img() {
	//
}
