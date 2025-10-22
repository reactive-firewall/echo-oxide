use clap::{Parser};

#[derive(Parser)]
#[command(name = "echo")]
#[command(about = "Output the ARGs.")]
#[command(long_about = "A command-line tool for echoing out arguments. This is the rust edition.")]
pub struct Args {
	/// Omit trailing newline character
	#[arg(short = 'n', long = "one-line")]
	pub omit_newline: bool,

	/// Enable handling of escape codes
	#[arg(short = 'e', long)]
	pub enable_escape: bool,

	/// Disable handling of escape codes
	#[arg(short = 'E', long)]
	pub disable_escape: bool,

	/// Input values or strings
	pub inputs: Vec<String>,
}

fn main() {
	let args = Args::parse();

		// Determine escape code handling behavior
	let handle_escape = if args.disable_escape {
		false
	} else {
		args.enable_escape
	};

	for arg in &args.inputs {
		let output = if handle_escape {
			handle_escape_codes(arg) // Function to handle escape codes
		} else {
			arg.to_string() // Just return the string as is
		};
		print!("{} ", output);
	}

	// Print a newline after all inputs if using println!
	if !args.omit_newline {
		println!(); // To move to the next line after all outputs
	}
}

/// Function to handle escape codes
fn handle_escape_codes(input: &str) -> String {
	let mut output = String::new();
	let mut chars = input.chars().peekable(); // Peekable iterator

	while let Some(c) = chars.next() {
		if c == '\\' { // Handle escape character
			match chars.next() { // Look at the next character
				Some('a') => output.push('\x07'),   // Alert (bell)
				Some('b') => output.push('\x08'),   // Backspace
				Some('c') => break,                  // Suppress trailing newline
				Some('E') => output.push('\x1B'),    // Escape character
				Some('f') => output.push('\x0C'),    // Form feed
				Some('n') => output.push('\n'),      // New line
				Some('r') => output.push('\r'),      // Carriage return
				Some('t') => output.push('\t'),      // Horizontal tab
				Some('v') => output.push('\x0B'),    // Vertical tab
				Some('\\') => output.push('\\'),      // Backslash
				Some(d) if d.is_digit(8) => {        // Octal handling
					let mut octal_str = String::new();
					octal_str.push(d);
					while let Some(&next) = chars.peek() {
						if next.is_digit(8) && octal_str.len() < 3 {
							octal_str.push(chars.next().unwrap());
						} else {
							break; // If not a digit, stop reading
						}
					}
					if let Ok(octal) = u8::from_str_radix(&octal_str, 8) {
						output.push(octal as char); // Convert octal to char
					}
				},
				_ => output.push(c), // Default: no escape, just push the character
			}
		} else {
			output.push(c); // Normal character, just push
		}
	}

	output
}
