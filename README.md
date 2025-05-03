# SequentialPass Number Generator

This project is a simple Rust-based command-line tool for generating large lists of phone numbers with specific prefixes. It is designed for educational and demonstration purposes.

## Features
- Interactive CLI menu for selecting number types
- Generates files with millions of sequential numbers for various prefixes:
  - BL 014
  - BL 019
  - GP 013
  - GP 017
  - ROBI 018
  - TELE 015
- Each selection creates a text file with numbers in the format: `<prefix><8-digit number>`

## Usage
1. **Build the project** (if needed):
   ```sh
   cargo build --release
   ```
2. **Run the program:**
   ```sh
   cargo run --release
   ```
3. **Follow the on-screen menu** to select the number type you want to generate.
4. The program will create a `.txt` file (e.g., `bl_014.txt`) in the current directory containing all numbers for the selected prefix.

## Output Example
For BL 014, the output file will contain lines like:
```
01400000000
01400000001
...
01499999999
```

## Requirements
- Rust toolchain (https://www.rust-lang.org/tools/install)
- Sufficient disk space for large output files

## License
This project is licensed under the GNU General Public License v3.0. See the LICENSE file for details.

## Disclaimer
This tool is for educational and demonstration purposes only. Do not use generated data for any unlawful or unethical activities.