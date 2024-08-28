# Crabodex

![Static Badge](https://img.shields.io/badge/Github-fabien--h%2Fcrabodex-dddddd?logo=github)
![Static Badge](https://img.shields.io/badge/licence-MIT-dddddd?logo=opensourceinitiative&logoColor=%23ffffff)
![Tests](https://img.shields.io/badge/tests-passing-green)
![Build](https://img.shields.io/badge/build-passing-green)


Crabodex is a documentation generator for Markdown files. It recursively processes Markdown files in a given directory and generates HTML documentation.

## Usage

`crabodex generate <input_directory> <output_directory>`

## Features

- Recursively processes Markdown files in the input directory
- Generates HTML output using customizable templates
- Supports hierarchical documentation structure

## License

This project is licensed under the MIT License.

### Work on the project

Run `cargo clippy --all-features --tests --benches -- -Dclippy::all -Dclippy::pedantic` before you commit!
