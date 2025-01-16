use std::io::{Read, Write};

pub fn renumber_field_numbers(
    proto_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_str = String::new();
    {
        let mut input_file = std::fs::File::open(proto_file)?;
        input_file.read_to_string(&mut input_str)?;
    }

    let mut output_lines = Vec::new();

    let message_start_pattern = regex::Regex::new(r"^\s*message\s+\w+\s*\{")?;
    let field_pattern = regex::Regex::new(r"^\s*(\w+)\s+(\w+)\s*=\s*\d+\s*;")?;

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
                let new_line = format!(
                    "  {} {} = {};",
                    field_type, field_name, current_field_number
                );
                output_lines.push(new_line);
                current_field_number += 1;
                continue;
            }
        }

        output_lines.push(line.to_owned());
    }

    let mut output = std::fs::File::create(output_file)?;
    output.write_all((output_lines.join("\n") + "\n").as_bytes())?;

    Ok(())
}
