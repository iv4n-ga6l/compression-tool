# Compression Tool

## Overview
The Compression Tool is a command-line utility for compressing and decompressing text files using Huffman coding. It efficiently reduces file sizes by encoding characters based on their frequency in the input file.

## Features
- **Compression**: Converts input text files into compressed binary format.
- **Decompression**: Restores compressed files back to their original text format.
- **Frequency Analysis**: Analyzes character frequencies in the input file.
- **Huffman Tree Construction**: Builds a binary tree for prefix code generation.
- **Header Serialization**: Stores character frequency data in the compressed file for decompression.

## Installation

### Prerequisites
- Rust (latest stable version)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/iv4n-ga6l/compression-tool.git
   cd compression-tool
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. The executable will be available in the `target/release` directory.

## Usage

### Compression
To compress a file:
```bash
./compression-tool <input_filename> <output_filename>
```
Example:
```bash
./compression-tool input.txt compressed.bin
```
This will generate a compressed file `compressed.bin` containing the header and compressed data.

### Decompression
To decompress a file:
```bash
./compression-tool <compressed_filename> <output_filename>
```
Example:
```bash
./compression-tool compressed.bin output.txt
```
This will restore the original text content into `output.txt`.

### Notes
- Ensure the input file exists and is readable.
- The output file will be overwritten if it already exists.

## Testing

### Running Tests
The repository includes comprehensive unit tests for all major components:
- Frequency analysis
- Huffman tree construction
- Header serialization and parsing
- Compression and decompression

To run the tests:
```bash
cargo test
```

### Example Test Cases
1. **Frequency Analysis**: Verify character frequency calculation for various input files.
2. **Header Parsing**: Ensure the header is correctly parsed from compressed files.
3. **Compression and Decompression**: Test the integrity of compressed and decompressed files.