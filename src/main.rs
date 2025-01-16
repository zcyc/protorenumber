use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use clap::{Arg, Command};

fn renumber_field_numbers(proto_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open(proto_file)?;
    let reader = io::BufReader::new(input);

    let mut output_lines = Vec::new();

    let message_start_pattern = Regex::new(r"^\s*message\s+\w+\s*\{")?;
    let field_pattern = Regex::new(r"^\s*(\w+)\s+(\w+)\s*=\s*\d+\s*;")?;

    let mut current_field_number = 1;
    let mut in_message_block = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if message_start_pattern.is_match(trimmed_line) {
            in_message_block = true;
            current_field_number = 1;
        } else if in_message_block && trimmed_line == "}" {
            in_message_block = false;
        }

        if in_message_block {
            if let Some(caps) = field_pattern.captures(trimmed_line) {
                let field_type = &caps[1];
                let field_name = &caps[2];
                let new_line = format!("    {} {} = {};", field_type, field_name, current_field_number);
                output_lines.push(new_line);
                current_field_number += 1;
                continue;
            }
        }

        output_lines.push(line);
    }

    let mut output = File::create(output_file)?;
    for line in output_lines {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let matches = Command::new("ProtoReorder")
        .version("1.0")
        .author("zcyc <8595764@qq.com>")
        .about("A Rust tool to renumber Protobuf field numbers.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to the input .proto file."),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to the output file with renumbered field numbers."),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    if let Err(e) = renumber_field_numbers(input_file, output_file) {
        eprintln!("Error processing proto file: {}", e);
    } else {
        println!("Renumber proto file written to {}", output_file);
    }
}