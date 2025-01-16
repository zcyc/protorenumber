use std::{cell::LazyCell, io::Read};

pub fn renumber_field_numbers(
    proto_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_str = String::new();
    {
        let mut input_file = std::fs::File::open(proto_file)?;
        input_file.read_to_string(&mut input_str)?;
    }

    let message_start_pattern: LazyCell<regex::Regex> =
        LazyCell::new(|| regex::Regex::new(r"^\s*message\s+\w+\s*\{").unwrap());
    let field_pattern: LazyCell<regex::Regex> =
        LazyCell::new(|| regex::Regex::new(r"^\s*(\w+)\s+(\w+)\s*=\s*\d+\s*;").unwrap());

    let line_count = input_str.lines().count();
    let mut output_lines = Vec::with_capacity(line_count);

    let mut current_field_number = 1;
    let mut in_message_block = false;

    for line in input_str.lines() {
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
                output_lines.push(format!(
                    "  {} {} = {};",
                    field_type, field_name, current_field_number
                ));
                current_field_number += 1;
                continue;
            }
        }

        output_lines.push(line.to_owned());
    }

    let output_string = output_lines.join("\n") + "\n";
    std::fs::write(output_file, output_string.into_bytes())?;

    Ok(())
}
