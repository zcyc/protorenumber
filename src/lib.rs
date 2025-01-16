use std::io::{BufRead, Write};

static MESSAGE_PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
static FIELD_PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

fn get_message_pattern() -> &'static regex::Regex {
    MESSAGE_PATTERN.get_or_init(|| regex::Regex::new(r"^\s*message\s+\w+\s*\{").unwrap())
}

fn get_field_pattern() -> &'static regex::Regex {
    FIELD_PATTERN.get_or_init(|| regex::Regex::new(r"^\s*(\w+)\s+(\w+)\s*=\s*\d+\s*;").unwrap())
}

pub fn renumber_field_numbers(
    proto_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::fs::File::open(proto_file)?;
    let reader = std::io::BufReader::new(input_file);

    let output_file = std::fs::File::create(output_file)?;
    let mut writer = std::io::BufWriter::new(output_file);

    let mut current_field_number = 1;
    let mut in_message_block = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if get_message_pattern().is_match(trimmed_line) {
            in_message_block = true;
            current_field_number = 1;
        } else if in_message_block && trimmed_line == "}" {
            in_message_block = false;
        }

        if in_message_block {
            if let Some(caps) = get_field_pattern().captures(trimmed_line) {
                let field_type = &caps[1];
                let field_name = &caps[2];
                writeln!(
                    writer,
                    "  {} {} = {};",
                    field_type, field_name, current_field_number
                )?;
                current_field_number += 1;
                continue;
            }
        }

        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;

    Ok(())
}
