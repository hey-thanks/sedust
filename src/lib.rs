use std::env;
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    pub script: String,
    pub filenames: Vec<String>, // Assume a filename is required. Probably change to an Option eventually?
}

impl Input {
    pub fn new(mut args: env::Args) -> Result<Input, &'static str> {
	args.next();

	let script = match args.next() {
	    Some(arg) => arg,
	    None => return Err("Missing at least one input."),
	};

	let mut filenames = vec![];
	for argument in args {
	    filenames.push(argument);
	}

	Ok(Input { script, filenames })
    }
}

pub struct Script {
    pub address: Option<String>,
        // Could a sinlge number, a range, or a regex ($ is last line)
	// No address means select every pattern space
	// One address shall select each pattern space the matches the address
        // Two addresses shall select an inclusive range
    pub command: char,
    pub options: Option<String>,
}

impl Script {
    // This should probably return an error if the script is ill-formatted
    pub fn new(script: &str) -> Script {
	let re = Regex::new(r"(^\d*|^/.*?/|^\d*,\d*|^\d*,/.*?/)([abcdDgGhHilnNpPqrstwxy:=#])(.*)").unwrap();
	let captures = re.captures(script).unwrap();
	// Not 100% sure what index 0 is?
	let address = &captures[1];
	let command = &captures[2];
	let options = &captures[3];

	println!("Address: {}", address);
	println!("Command: {:?}", command);
	println!("Options: {:?}", options);

	Script{ address: Some(address.to_string()),
		command: command.chars().next().unwrap(),
		options: Some(options.to_string()) }
    }
}
