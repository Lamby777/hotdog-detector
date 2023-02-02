// Call lib.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
	rust_cli_template::main(
		std::env::args().collect::<Vec<String>>()
	)
}
