fn main() {
    let matches = clap::Command::new("ProtoRenumber")
        .version("1.0")
        .author("zcyc <8595764@qq.com>")
        .about("A Rust tool to renumber Protobuf field numbers")
        .arg(
            clap::Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to the input .proto file"),
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to the output file with renumbered field numbers"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    if let Err(e) = protorenumber::renumber_field_numbers(input_file, output_file) {
        eprintln!("Error processing proto file: {}", e);
    } else {
        println!("Renumber proto file written to: {}", output_file);
    }
}
