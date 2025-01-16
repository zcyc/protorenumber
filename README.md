# ProtoRenumber

ProtoRenumber is a command-line tool written in Rust that renumbers the fields in Protobuf (`.proto`) files while preserving the original field order.

The tool is particularly useful for developers looking to clean up or reorganize Protobuf files by assigning sequential field numbers starting from 1.

## Features

- Automatically renumber field numbers in Protobuf files (starting from 1 and incrementing sequentially)
- Preserves the original order of fields in the Protobuf file
- Simple and user-friendly CLI interface

## Install

```bash
cargo install --git https://github.com/zcyc/ProtoRenumber
```

## Usage

### Command Syntax

```bash
ProtoRenumber --input <input_file> --output <output_file>
```

### Arguments

- `--input` (or `-i`): The path to the input `.proto` file _(Required)_
- `--output` (or `-o`): The path to save the renumbered Protobuf file _(Required)_

### Example

```bash
ProtoRenumber --input example.proto --output output.proto
```

Expected output:

```
Renumber proto file written to output.proto
```

## Example Input and Output

### Input File (`example.proto`)

```proto
syntax = "proto3";

message User {
  string name = 3;
  int32 age = 1;
  string email = 4;
}

message Product {
  int32 id = 2;
  string description = 6;
  double price = 3;
}
```

### Output File (`output.proto`)

```proto
syntax = "proto3";

message User {
  string name = 1;
  int32 age = 2;
  string email = 3;
}

message Product {
  int32 id = 1;
  string description = 2;
  double price = 3;
}
```

## License

[MIT](LICENSE)
