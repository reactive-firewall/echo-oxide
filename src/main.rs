use clap::{Parser};
use echo-oxide::Args;
use echo-oxide::echo_cmd;

fn main() {
	let args = Args::parse();

	echo_cmd(&args);

	// Print a newline after all inputs if using println!
	if !args.omit_newline {
		println!(); // To move to the next line after all outputs
	}
}
