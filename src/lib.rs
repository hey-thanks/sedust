use regex::Regex;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Debug)]
pub struct Input {
    pub script: String,
    pub filenames: Option<Vec<String>>,
    pub suppress_printing: bool,
}

impl Input {
    pub fn new() -> Result<Input, &'static str> {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let e_scripts: Option<Vec<_>> = match matches.values_of("e") {
            Some(scripts) => Some(scripts.collect()),
            None => None,
	};
	
	let f_scripts: Option<Vec<_>> = match matches.values_of("f") {
            Some(files) => Some(files.collect()),
            None => None,
	};
	
	let mut f_in_vec = None;
	let mut indexed_input_script = None;
	
	if e_scripts.is_none() && f_scripts.is_none() {
            if let Some(script) = matches.value_of("input_script") {
		indexed_input_script = Some(script);
            }
	} else {
            if let Some(script) = matches.value_of("input_script") {
		f_in_vec = Some(vec![script.to_string()]);
            }
	}
	
	if let Some(filenames) = matches.values_of("f_in") {
	    if f_in_vec.is_none() {
		f_in_vec = Some(filenames.map(|x| x.to_string()).collect());
	    } else {
		f_in_vec.as_mut().unwrap().append(&mut filenames.map(|x| x.to_string()).collect());
	    }
	}

	let script = match indexed_input_script {
	    Some(thing) => thing,
	    None => e_scripts.unwrap()[0],
	};

        Ok(Input {
            script: script.to_string(),
	    filenames: f_in_vec,
	    suppress_printing: matches.is_present("n"),
        })
    }
}

#[derive(Debug)]
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
        let re = Regex::new(r"(^\d*|^/.*?/|^\d*,\d*|^\d*,/.*?/)([abcdDgGhHilnNpPqrstwxy:=#])(.*)")
            .unwrap();
        let captures = re.captures(script).unwrap();
        // Not 100% sure what index 0 is?
        let address = &captures[1];
        let command = &captures[2];
        let options = &captures[3];

        // println!("Address: {:?}", address);
        // println!("Command: {:?}", command);
        // println!("Options: {:?}", options);

        Script {
            address: if address.to_string().is_empty() {
                None
            } else {
                Some(address.to_string())
            },
            command: command.chars().next().unwrap(),
            options: Some(options.to_string()),
        }
    }
}
