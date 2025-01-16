# ProtoRenumber

ProtoRenumber is a command-line tool written in Rust that renumbers the fields in Protobuf (`.proto`) files while preserving the original field order.
The tool is particularly useful for developers looking to clean up or reorganize Protobuf files by assigning sequential field numbers starting from 1.

---

## Features

- Automatically renumber field numbers in Protobuf files (starting from 1 and incrementing sequentially).
- Preserves the original order of fields in the Protobuf file.
- Simple and user-friendly CLI interface.

---

## Requirements

- Rust 1.65 or later (latest version recommended).

---

## Installation

To install and set up **ProtoRenumber**, follow these steps:

### Step 1: Install Rust

If Rust is not already installed, you can download and install it from the [official Rust website](https://www.rust-lang.org/).
Alternatively, use this one-liner to install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

After installation, verify that Rust is installed correctly:

```bash
rustc --version
```

### Step 2: Clone and Build the Project

1. Clone the GitHub repository:
   ```bash
   git clone https://github.com/zcyc/ProtoRenumber.git
   cd ProtoRenumber
   ```

2. Compile the project in release mode:
   ```bash
   cargo build --release
   ```

3. The compiled executable will be available in the `target/release` directory:
   ```bash
   ./target/release/ProtoRenumber
   ```

---

## Usage

### Command Syntax

```bash
ProtoRenumber --input <input_file> --output <output_file>
```

### Arguments

- `--input` (or `-i`): The path to the input `.proto` file.
  *(Required)*

- `--output` (or `-o`): The path to save the renumbered Protobuf file.
  *(Required)*

### Example

To process a file located at `example.proto` and save the renumbered output to `renumbered_example.proto`:

```bash
./target/release/ProtoRenumber --input example.proto --output renumbered_example.proto
```

Expected output:

```plaintext
Renumbered proto file written to renumbered_example.proto
```

---

## Example Input and Output

### Input File (`example.proto`)

```proto
message Example {
    string name = 3;
    int32 age = 1;
    repeated string tags = 7;
    bool active = 2;
}
```

### Output File (`renumbered_example.proto`)

After processing with `ProtoRenumber`, the fields will be renumbered sequentially:

```proto
message Example {
    string name = 1;
    int32 age = 2;
    repeated string tags = 3;
    bool active = 4;
}
```

---

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b my-new-feature
   ```
3. Commit your changes:
   ```bash
   git commit -am 'Add some feature'
   ```
4. Push your branch to your forked repository:
   ```bash
   git push origin my-new-feature
   ```
5. Create a pull request.

---

## License

ProtoRenumber is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
