use echo-oxide::Args;
use echo-oxide::handle_escape_codes; // Replace with your actual crate name
use echo-oxide::parse_handle_escape;

#[test]
fn test_escape_codes() {
	assert_eq!(handle_escape_codes("Hello\\nWorld"), "Hello\nWorld");
	assert_eq!(handle_escape_codes("Backspace\\b"), "Backspace\u{8}");
	assert_eq!(handle_escape_codes("Tab\\tSpace"), "Tab\tSpace");
	assert_eq!(handle_escape_codes("Bell\\aSound"), "Bell\x07Sound");
	assert_eq!(handle_escape_codes("Octal \\075"), "Octal ="); // = ASCII 61 in octal 75
}

#[test]
fn test_no_escape() {
	assert_eq!(handle_escape_codes("No escape"), "No escape");
}

#[test]
fn test_multiple_escapes() {
	assert_eq!(handle_escape_codes("Line 1\\nLine 2"), "Line 1\nLine 2");
	assert_eq!(handle_escape_codes("Tabbed\\tText"), "Tabbed\tText");
}

#[test]
fn test_enable_escape() {
	let args = Args { omit_newline: false,
		enable_escape: true, disable_escape: false,
		inputs: None
	};
	assert!(parse_handle_escape(&args));
}

#[test]
fn test_disable_escape() {
	let args = Args { omit_newline: false,
		enable_escape: false, disable_escape: true,
		inputs: None
	};
	assert!(!parse_handle_escape(&args));
}

#[test]
fn test_no_flags() {
	let args = Args { omit_newline: false,
		enable_escape: false, disable_escape: false,
		inputs: None
	};
	assert!(!parse_handle_escape(&args));
}

#[test]
fn test_conflicting_flags() {
	let args = Args { omit_newline: false,
		enable_escape: true, disable_escape: true,
		inputs: None
	};
	assert!(!parse_handle_escape(&args));
}
