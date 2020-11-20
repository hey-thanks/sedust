use regex::Regex;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::process;

use sedust::Input;
use sedust::Script;

fn main() {
    let input = Input::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let script = Script::new(&input.script);
    println!("{:?}", script);

    let mut hold_space = String::new();
    let mut pattern_space = String::new();

    let mut buf_readers = Vec::new();
    for f_in in &input.filenames {
        let file = File::open(f_in).unwrap();
        buf_readers.push(BufReader::new(file));
    }

    let mut begin_address: usize = 0;
    let mut end_address: usize = usize::MAX;
    match script.address {
        Some(address) => {
            let address_range = address.split(',').collect::<Vec<&str>>();
            begin_address = address_range[0].parse().unwrap();
            end_address = begin_address;
            if address_range.len() == 2 {
                end_address = address_range[1].parse().unwrap();
            }
        }
        _ => {}
    }

    println!("--- Output ---");
    // Only deal with numerical addresses for now
    let options = &script.options.unwrap();
    let mut index = 0;
    for reader in buf_readers {
        for line in reader.lines() {
            index += 1;
            pattern_space = line.unwrap();

            if index >= begin_address && index <= end_address {
                match script.command {
                    'a' => {
                        pattern_space.push('\n');
                        pattern_space.push_str(&options);
                    }

                    'c' => {
                        if index == end_address {
                            pattern_space = options.to_string();
                        } else {
                            pattern_space.clear();
                        }
                    }

                    'd' => pattern_space.clear(),

                    'g' => pattern_space = hold_space.clone(),

                    'G' => {
                        pattern_space.push('\n');
                        pattern_space.push_str(&hold_space);
                    }

                    'h' => hold_space = pattern_space.clone(),

                    'H' => {
                        hold_space.push('\n');
                        hold_space.push_str(&pattern_space);
                    }

                    'i' => {
                        let mut temp = String::new();
                        temp.push_str(&options);
                        temp.push('\n');
                        temp.push_str(&pattern_space);
                        pattern_space = temp;
                    }

                    'p' => println!("{}", pattern_space),

                    'P' => println!("{}", pattern_space.split('\n').collect::<Vec<&str>>()[0]),

                    'q' => {
                        // This is probably a hack
                        println!("{}", pattern_space);
                        return;
                    }

                    'r' => {
                        println!("{}", pattern_space);
                        let r_file = File::open(&options).unwrap();
                        let mut r_buf_reader = BufReader::new(r_file);
                        for r_line in r_buf_reader.lines() {
                            println!("{}", r_line.unwrap());
                        }
                    }

                    'w' => {
                        let mut f_out = OpenOptions::new().append(true).open(&options).unwrap();

                        writeln!(f_out, "{}", pattern_space);
                    }

                    'x' => {
                        let mut temp = String::new();
                        temp = pattern_space;
                        pattern_space = hold_space;
                        hold_space = temp;
                    }

                    '=' => println!("{}", index),

                    '#' => continue,

                    _ => panic!("Command not recognized. Aborting."),
                }
            }

            if script.command == 'r' && index == begin_address {
                // Don't print because the pattern space is printed
                // /before/ the r command does its thing
            } else if !pattern_space.is_empty() {
                println!("{}", pattern_space);
            } else if pattern_space.is_empty() && (script.command == 'x' || script.command == 'g') {
                println!("{}", pattern_space);
            }
        }
    }
}

// // Commands can be separated by semicolons (;)
// pub enum Command {
//     a, // (a text) append text after line
//     b, // (b label) branch unconditionally to label
//     c, // (c text) replace (change) lines with text
//     d, // delete pattern space and immediately start next cycle
//     D, // if no newline in pattern space, delete line. Else, delete up
//     // to newline, start next cycle with resultant pattern space, without reading additional input
//     g, // replace the contents of the pattern space with the contents of the hold space
//     G, // append a newline to the pattern space followed by the contents of the hold space
//     h, // replace the contents of the hold space with the contents of the pattern space
//     H, // append a newline to the hold space followed by the contents
//     // of the pattern space
//     i, // (i text) insert text before line
//     l, // print the pattern space in an unambiguous form
//     n, // write the pattern space to standard output if the default
//     // output has not been suppressed, and replace the pattern space with
//     // the next line of input, less its terminating newline.
//     N, // Append the next line of input, less its terminating
//     // <newline>, to the pattern space, using an embedded <newline> to
//     // separate the appended material from the original material. Note
//     // that the current line number changes.
//     p, // write the pattern space to stdout
//     P, // write the pattern space, up to the first newline, to stdout
//     q, // branch to the end of the script without starting a new cycle
//     r, // (r rfile) copy the contents of rfile to stdout
//     s, // (s/BRE/replacement) Substitute the replacement string for
//     // instances of the BRE in the pattern space.
//     t, // (t[label]) Test. Branch to the : command verb bearing the
//     // label if any substitutions have been made since the most recent
//     // reading of an input line or execution of a t. If label is not
//     // specified, branch to the end of the script.
//     w, // (w wfile) append the pattern space to wfile
//     x, // swap the contents of the pattern space and hold space
//     y, // (y/string1/string2) Replace all occurrences of characters in
//     // string1 with the corresponding characters in string2.
//     colon, // (:label) Do nothing. This command bears a label to which
//     // the b and t commands branch.
//     equals, // write the current line number followed by a newline to
//     // stdout
//     hash, // comment character
// }
