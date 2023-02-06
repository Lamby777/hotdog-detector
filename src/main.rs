// Call lib.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
	hotdog_detector::main(
		std::env::args().collect::<Vec<String>>()
	)
}
