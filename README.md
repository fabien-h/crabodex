# Crabodex

![Static Badge](https://img.shields.io/badge/Github-fabien--h%2Fcrabodex-dddddd?logo=github)
![Static Badge](https://img.shields.io/badge/licence-MIT-dddddd?logo=opensourceinitiative&logoColor=%23ffffff)
![Tests](https://img.shields.io/badge/tests-failing-red)
![Build](https://img.shields.io/badge/build-failing-red)
![Miri](https://img.shields.io/badge/miri-failing-red)
![Clippy](https://img.shields.io/badge/clippy-failing-red)
![Dependencies](https://img.shields.io/badge/dependencies-outdated-red)


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
